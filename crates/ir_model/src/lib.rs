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

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Instruction {
    Load {
        function: String,
        dst: String,
        src: String,
    },
    Store {
        function: String,
        dst: String,
        value: String,
    },
    Add {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    FAdd {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    FMul {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    Phi {
        function: String,
        dst: String,
        incoming: Vec<(String, String)>,
    },
    ICmp {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
        op: String,
    },
    GetElementPtr {
        function: String,
        dst: String,
        base: String,
        index: String,
    },
    Alloca {
        function: String,
        dst: String,
        ty: String,
        align: u32,
    },
    Br {
        function: String,
        cond: Option<String>,
        target_true: String,
        target_false: Option<String>,
    },
    Ret {
        function: String,
    },
    Sub {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    FSub {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    Mul {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    UDiv {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    SDiv {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    URem {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    SRem {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    FDiv {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    FRem {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
    },
    FCmp {
        function: String,
        dst: String,
        lhs: String,
        rhs: String,
        op: String,
    },
    Unhandled {
        function: String,
        text: String,
    },
}

impl Instruction {
    pub fn function_name(&self) -> &str {
        match self {
            Instruction::Load { function, .. }
            | Instruction::Store { function, .. }
            | Instruction::Add { function, .. }
            | Instruction::FAdd { function, .. }
            | Instruction::FMul { function, .. }
            | Instruction::Phi { function, .. }
            | Instruction::ICmp { function, .. }
            | Instruction::GetElementPtr { function, .. }
            | Instruction::Alloca { function, .. }
            | Instruction::Br { function, .. }
            | Instruction::Ret { function, .. }
            | Instruction::Sub { function, .. }
            | Instruction::FSub { function, .. }
            | Instruction::Mul { function, .. }
            | Instruction::UDiv { function, .. }
            | Instruction::SDiv { function, .. }
            | Instruction::URem { function, .. }
            | Instruction::SRem { function, .. }
            | Instruction::FDiv { function, .. }
            | Instruction::FRem { function, .. }
            | Instruction::FCmp { function, .. }
            | Instruction::Unhandled { function, .. } => function,
        }
    }

    pub fn used_operands(&self) -> Vec<&str> {
        use Instruction::*;

        match self {

            // Arithmetic 3 operands
            Add { dst, lhs, rhs, .. }
            | Sub { dst, lhs, rhs, .. }
            | Mul { dst, lhs, rhs, .. }
            | UDiv { dst, lhs, rhs, .. }
            | SDiv { dst, lhs, rhs, .. }
            | URem { dst, lhs, rhs, .. }
            | SRem { dst, lhs, rhs, .. }
            | FAdd { dst, lhs, rhs, .. }
            | FSub { dst, lhs, rhs, .. }
            | FMul { dst, lhs, rhs, .. }
            | FDiv { dst, lhs, rhs, .. }
            | FRem { dst, lhs, rhs, .. }
            | ICmp { dst, lhs, rhs, .. }
            | FCmp { dst, lhs, rhs, .. } => vec![dst, lhs, rhs],

            // Load/Store
            Load { dst, src, .. } => vec![dst, src],
            Store { dst, value, .. } => vec![dst, value],

            // Alloca
            Alloca { dst, .. } => vec![dst],

            // GEP
            GetElementPtr {
                dst, base, index, ..
            } => vec![dst, base, index],

            // Phi
            Phi { dst, incoming, .. } => {
                let mut v = vec![dst.as_str()];
                for (label, val) in incoming {
                    v.push(label);
                    v.push(val);
                }
                v
            }

            // Other
            Unhandled { text, .. } => vec![text],
            _ => vec![],
        }
    }
}
