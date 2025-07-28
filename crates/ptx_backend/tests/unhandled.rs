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
use ptx_backend::{declare_registers, to_ptx};

#[test]
fn test_unhandled_instruction_output() {
    let instrs = vec![
        Instruction::Alloca {
            dst: "%i".into(),
            ty: "i32".into(),
            align: 0,
        },
        Instruction::Unhandled("foobar %a, %b".into()),
    ];

    let mut actual = String::new();

    actual.push_str(".version 7.0\n");
    actual.push_str(".target sm_75\n");
    actual.push_str(".address_size 64\n\n");

    actual.push_str("// Function: test\n");
    actual.push_str("// Block: entry\n");

    let instr_refs: Vec<&_> = instrs.iter().collect();
    actual.push_str(&declare_registers(&instr_refs));
    actual.push_str(".entry test {\n");

    for instr in &instrs {
        actual.push_str(&format!("    {}\n", to_ptx(instr)));
    }

    actual.push_str("}\n");

    insta::assert_snapshot!("unhandled_instr", actual);
}
