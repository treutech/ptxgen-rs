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
use llvm_ir::Module;
use llvm_parser::convert::lower;

use serde_json;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: llvm_parser <file.ll>");

    let ll_text = fs::read_to_string(filename).expect("Failed to read .ll file");
    let module = Module::from_ir_str(&ll_text).expect("Failed to parse LLVM IR");

    for func in &module.functions {
        println!("Function: {}", func.name);
        for block in &func.basic_blocks {
            println!("  Basic block: {}", block.name);
            for instr in &block.instrs {
                let lowered: Instruction = lower(&func.name, instr);
                println!("{}", serde_json::to_string_pretty(&lowered).unwrap());
            }
        }
    }
}
