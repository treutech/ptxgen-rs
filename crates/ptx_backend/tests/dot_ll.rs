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
use ptx_backend::lower_to_ptx;

#[test]
fn test_dot_ptx_output() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("dot.ll");

    let module = parse_module(&path).expect("Failed to parse module");

    let mut actual = String::new();

    for func in module.functions {
        actual.push_str(&format!("// Function: {}\n", func.name));
        for block in func.basic_blocks {
            actual.push_str(&format!("// Block: {}\n", block.name));
            let instrs = block
                .instrs
                .iter()
                .map(llvm_parser::convert::lower)
                .collect::<Vec<_>>();
            for line in lower_to_ptx(&instrs) {
                actual.push_str(&format!("{}\n", line));
            }
        }
    }

    insta::assert_snapshot!("dot_ptx", actual);
}
