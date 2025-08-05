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
        FCmp(cmp) => Instruction::FCmp {
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
        Sub(s) => Instruction::Sub {
            function: function.to_string(),
            dst: s.dest.to_string(),
            lhs: s.operand0.to_string(),
            rhs: s.operand1.to_string(),
        },
        FSub(s) => Instruction::FSub {
            function: function.to_string(),
            dst: s.dest.to_string(),
            lhs: s.operand0.to_string(),
            rhs: s.operand1.to_string(),
        },
        Mul(m) => Instruction::Mul {
            function: function.to_string(),
            dst: m.dest.to_string(),
            lhs: m.operand0.to_string(),
            rhs: m.operand1.to_string(),
        },
        UDiv(d) => Instruction::UDiv {
            function: function.to_string(),
            dst: d.dest.to_string(),
            lhs: d.operand0.to_string(),
            rhs: d.operand1.to_string(),
        },
        SDiv(d) => Instruction::SDiv {
            function: function.to_string(),
            dst: d.dest.to_string(),
            lhs: d.operand0.to_string(),
            rhs: d.operand1.to_string(),
        },
        URem(r) => Instruction::URem {
            function: function.to_string(),
            dst: r.dest.to_string(),
            lhs: r.operand0.to_string(),
            rhs: r.operand1.to_string(),
        },
        SRem(r) => Instruction::SRem {
            function: function.to_string(),
            dst: r.dest.to_string(),
            lhs: r.operand0.to_string(),
            rhs: r.operand1.to_string(),
        },
        FDiv(d) => Instruction::FDiv {
            function: function.to_string(),
            dst: d.dest.to_string(),
            lhs: d.operand0.to_string(),
            rhs: d.operand1.to_string(),
        },
        FRem(r) => Instruction::FRem {
            function: function.to_string(),
            dst: r.dest.to_string(),
            lhs: r.operand0.to_string(),
            rhs: r.operand1.to_string(),
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
        Select(sel) => Instruction::Select {
            function: function.to_string(),
            dst: sel.dest.to_string(),
            cond: sel.condition.to_string(),
            val_true: sel.true_value.to_string(),
            val_false: sel.false_value.to_string(),
        },
        BitCast(bc) => Instruction::Bitcast {
            function: function.to_string(),
            dst: bc.dest.to_string(),
            src: bc.operand.to_string(),
        },
        ZExt(z) => Instruction::ZExt {
            function: function.to_string(),
            dst: z.dest.to_string(),
            src: z.operand.to_string(),
        },
        Trunc(t) => Instruction::Trunc {
            function: function.to_string(),
            dst: t.dest.to_string(),
            src: t.operand.to_string(),
        },
        Call(c) => {
            let target = match &c.function {
                either::Either::Right(llvm_ir::Operand::ConstantOperand(const_ref)) => {
                    match const_ref.as_ref() {
                        llvm_ir::constant::Constant::GlobalReference { name, .. } => match name {
                            llvm_ir::Name::Name(s) => s.to_string(),
                            llvm_ir::Name::Number(n) => format!("{}", n),
                        },
                        _ => "unknown_fn".to_string(),
                    }
                }
                _ => "unknown_fn".to_string(),
            };

            Instruction::Call {
                function: function.to_string(),
                target,
                args: c.arguments.iter().map(|a| a.0.to_string()).collect(),
            }
        }
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
