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
        dst: String,
        src: String,
    },
    Store {
        dst: String,
        value: String,
    },
    Add {
        dst: String,
        lhs: String,
        rhs: String,
    },
    FAdd {
        dst: String,
        lhs: String,
        rhs: String,
    },
    FMul {
        dst: String,
        lhs: String,
        rhs: String,
    },
    ICmp {
        dst: String,
        lhs: String,
        rhs: String,
        op: String,
    },
    GetElementPtr {
        dst: String,
        base: String,
        index: String,
    },
    Alloca {
        dst: String,
        ty: String,
        align: u32,
    },
    Br {
        cond: Option<String>,
        target_true: String,
        target_false: Option<String>,
    },
    Ret,
    Unhandled(String),
}

impl Instruction {
    pub fn used_operands(&self) -> Vec<&str> {
        use Instruction::*;
        match self {
            FMul { dst, lhs, rhs }
            | FAdd { dst, lhs, rhs }
            | Add { dst, lhs, rhs }
            | ICmp { dst, lhs, rhs, .. } => vec![dst, lhs, rhs],

            Load { dst, src } => vec![dst, src],
            Store { dst, value } => vec![dst, value],

            Alloca { dst, .. } => vec![dst],
            GetElementPtr { dst, base, index } => vec![dst, base, index],

            Unhandled(s) => vec![s],
            _ => vec![],
        }
    }
}
