// Copyright 2025 Raul Estrada <restrada@treutech.io>
// SPDX-License-Identifier: Apache-2.0
//
// This file is part of the PTXGEN-RS project by Treu Technologies.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ir_model::Instruction;
use llvm_ir::instruction::Instruction as LlvmInst;

pub fn lower(function: &str, instr: &LlvmInst) -> Instruction {
    use LlvmInst::*;
    match instr {
        FMul(f) => Instruction::FMul {
            function: function.to_string(),
            dst: f.dest.to_string(),
            lhs: f.operand0.to_string(),
            rhs: f.operand1.to_string(),
        },
        FAdd(f) => Instruction::FAdd {
            function: function.to_string(),
            dst: f.dest.to_string(),
            lhs: f.operand0.to_string(),
            rhs: f.operand1.to_string(),
        },
        Load(l) => Instruction::Load {
            function: function.to_string(),
            dst: l.dest.to_string(),
            src: l.address.to_string(),
        },
        Store(s) => Instruction::Store {
            function: function.to_string(),
            dst: s.address.to_string(),
            value: s.value.to_string(),
        },
        Alloca(a) => Instruction::Alloca {
            function: function.to_string(),
            dst: a.dest.to_string(),
            ty: format!("{:?}", a.allocated_type),
            align: a.alignment,
        },
        ICmp(cmp) => Instruction::ICmp {
            function: function.to_string(),
            dst: cmp.dest.to_string(),
            op: format!("{:?}", cmp.predicate),
            lhs: cmp.operand0.to_string(),
            rhs: cmp.operand1.to_string(),
        },
        Add(add) => Instruction::Add {
            function: function.to_string(),
            dst: add.dest.to_string(),
            lhs: add.operand0.to_string(),
            rhs: add.operand1.to_string(),
        },
        Phi(p) => Instruction::Phi {
            function: function.to_string(),
            dst: p.dest.to_string(),
            incoming: p
                .incoming_values
                .iter()
                .map(|(val, label)| {
                    (
                        match label {
                            llvm_ir::Name::Name(s) => s.as_str().to_string(),
                            llvm_ir::Name::Number(n) => format!("{}", n),
                        },
                        val.to_string(),
                    )
                })
                .collect(),
        },

        GetElementPtr(gep) => Instruction::GetElementPtr {
            function: function.to_string(),
            dst: gep.dest.to_string(),
            base: gep.address.to_string(),
            index: gep
                .indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        },
        _ => Instruction::Unhandled {
            function: function.to_string(),
            text: format!("{:?}", instr),
        },
    }
}

use llvm_ir::Terminator;

pub fn lower_terminator(func: &str, term: &Terminator) -> Instruction {
    match term {
        Terminator::Ret(_) => Instruction::Ret {
            function: func.to_string(),
        },
        Terminator::Br(br) => Instruction::Br {
            function: func.to_string(),
            cond: None,
            target_true: br.dest.to_string(),
            target_false: None,
        },
        Terminator::CondBr(br) => Instruction::Br {
            function: func.to_string(),
            cond: Some(format!("{}", br.condition)),
            target_true: br.true_dest.to_string(),
            target_false: Some(br.false_dest.to_string()),
        },
        _ => Instruction::Unhandled {
            function: func.to_string(),
            text: format!("{:?}", term),
        },
    }
}
