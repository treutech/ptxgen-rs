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

/// Clean LLVM operand string for PTX emission.
///
/// This strips decorations (`%`, `@`), removes type annotations and
/// pointer indicators, and trims spaces.
///
/// Examples:
/// - `"i32* %i"` → `"i"`
/// - `"float* %x"` → `"x"`
/// - `"  f32* %y "` → `"y"`
pub fn clean_operand(op: &str) -> String {
    let mut s = op
        .trim()
        .split_whitespace()
        .last()
        .unwrap_or(op)
        .trim_start_matches('%')
        .trim_start_matches('@')
        .to_string();

    for pat in ["*", "i32", "float", "f32", "ptr", " "] {
        s = s.replace(pat, "");
    }
    s
}

pub fn get_register_type(instr: &Instruction, name: &str) -> Option<&'static str> {
    use Instruction::*;
    let matches = |s: &str| clean_operand(s) == clean_operand(name);

    match instr {
        FMul { dst, lhs, rhs, .. } | FAdd { dst, lhs, rhs, .. }
            if matches(dst) || matches(lhs) || matches(rhs) =>
        {
            Some("f32")
        }

        Load { dst, src, .. } if matches(dst) || matches(src) => Some("f32"),

        Store { dst, value, .. } if matches(dst) || matches(value) => Some("f32"),

        Add { dst, lhs, rhs, .. } if matches(dst) || matches(lhs) || matches(rhs) => Some("s32"),

        ICmp { lhs, rhs, .. } if matches(lhs) || matches(rhs) => Some("s32"),

        ICmp { dst, .. } if matches(dst) => Some("pred"),

        _ => None,
    }
}
