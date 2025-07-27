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

pub fn lower(instr: &LlvmInst) -> Instruction {
    use LlvmInst::*;
    match instr {
        FMul(f) => Instruction::FMul {
            dst: f.dest.to_string(),
            lhs: f.operand0.to_string(),
            rhs: f.operand1.to_string(),
        },
        FAdd(f) => Instruction::FAdd {
            dst: f.dest.to_string(),
            lhs: f.operand0.to_string(),
            rhs: f.operand1.to_string(),
        },
        Load(l) => Instruction::Load {
            dst: l.dest.to_string(),
            src: l.address.to_string(),
        },
        Store(s) => Instruction::Store {
            dst: s.address.to_string(),
            value: s.value.to_string(),
        },
        Alloca(a) => Instruction::Alloca {
            dst: a.dest.to_string(),
            ty: format!("{:?}", a.allocated_type),
            align: a.alignment,
        },
        ICmp(cmp) => Instruction::ICmp {
            dst: cmp.dest.to_string(),
            op: format!("{:?}", cmp.predicate),
            lhs: cmp.operand0.to_string(),
            rhs: cmp.operand1.to_string(),
        },
        Add(add) => Instruction::Add {
            dst: add.dest.to_string(),
            lhs: add.operand0.to_string(),
            rhs: add.operand1.to_string(),
        },
        GetElementPtr(gep) => Instruction::GetElementPtr {
            dst: gep.dest.to_string(),
            base: gep.address.to_string(),
            index: gep
                .indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        },
        _ => Instruction::Unhandled(format!("{:?}", instr)),
    }
}
