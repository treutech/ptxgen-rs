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
use ir_model::Instruction;

fn run_test(filename: &str) -> String {
    use llvm_parser::convert::lower;
    use ptx_backend::lower_function;

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(filename);
    let module = parse_module(&path).expect("Failed to parse module");

    let mut output = String::new();

    for func in module.functions {
        let all_instrs: Vec<(String, Vec<Instruction>)> = func
            .basic_blocks
            .into_iter()
            .map(|block| {
                let instrs = block.instrs.iter().map(lower).collect::<Vec<_>>();
                (block.name.to_string(), instrs)
            })
            .collect();
        
        for line in lower_function(&func.name, &all_instrs, "sm_75") {
            output.push_str(&format!("{}\n", line));
        }
    }

    output
}

#[test]
fn test_ret_only() {
    insta::assert_snapshot!("ret_only", run_test("ret_only.ll"));
}

#[test]
fn test_br_uncond() {
    insta::assert_snapshot!("br_uncond", run_test("br_uncond.ll"));
}

#[test]
fn test_br_cond() {
    insta::assert_snapshot!("br_cond", run_test("br_cond.ll"));
}

#[test]
fn test_phi_basic() {
    insta::assert_snapshot!("phi_basic", run_test("phi_basic.ll"));
}

#[test]
fn test_conflict_type() {
    insta::assert_snapshot!("conflict_type", run_test("conflict_type.ll"));
}
