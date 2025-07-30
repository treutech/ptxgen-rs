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

use llvm_parser::parse_module::parse_module;
use ptx_backend::{declare_registers, to_ptx};

#[test]
fn test_add_ptx_output() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("add.ll");

    let module = parse_module(&path).expect("Failed to parse module");

    let mut actual = String::new();

    // Emit global header once
    actual.push_str(".version 7.0\n");
    actual.push_str(".target sm_75\n");
    actual.push_str(".address_size 64\n\n");

    for func in module.functions {
        actual.push_str(&format!("// Function: {}\n", func.name));

        let mut instrs = vec![];

        for block in &func.basic_blocks {
            actual.push_str(&format!("// Block: {}\n", block.name));
            instrs.extend(
                block
                    .instrs
                    .iter()
                    .map(|i| llvm_parser::convert::lower(&func.name, i)),
            );
        }

        let instr_refs: Vec<&_> = instrs.iter().collect();
        actual.push_str(&declare_registers(&instr_refs));
        actual.push_str(&format!(".entry {} {{\n", func.name));

        for instr in &instrs {
            actual.push_str(&format!("    {}\n", to_ptx(instr)));
        }

        actual.push_str("}\n\n");
    }

    insta::assert_snapshot!("add_ptx", actual);
}
