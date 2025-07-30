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

use llvm_parser::convert::lower;
use llvm_parser::parse_module::parse_module;
use ptx_backend::lower_function;

fn check_single_entry(ptx: &str) -> bool {
    ptx.matches(".entry ").count() == 1
}

fn check_single_header(ptx: &str) -> bool {
    ptx.matches(".version").count() == 1
        && ptx.matches(".target").count() == 1
        && ptx.matches(".address_size").count() == 1
}

fn check_register_collisions(ptx: &str) -> bool {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for line in ptx.lines().filter(|l| l.trim().starts_with(".reg")) {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 3 {
            continue;
        }

        let reg_type = parts[1]; // .f32, .s32, etc.
        let regs = parts[2..].join(" ");
        let regs = regs
            .trim_end_matches(';')
            .split(',')
            .map(|r| r.trim_start_matches('%').trim());

        for r in regs {
            if let Some(prev) = map.insert(r.to_string(), reg_type.to_string()) {
                if prev != reg_type {
                    return false; // type conflict
                }
            }
        }
    }
    true
}

fn check_terminal_instruction(ptx: &str) -> bool {
    let mut last_instr = "";

    for line in ptx.lines().rev() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with(".") {
            continue;
        }
        if line == "}" {
            continue;
        }
        last_instr = line;
        break;
    }

    last_instr == "ret;" || last_instr.starts_with("bra ")
}

#[test]
fn validate_structural_rules() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("saxpy.ll");

    let module = parse_module(&path).expect("Failed to parse module");

    let mut ptx = String::new();

    for func in module.functions {
        let all_instrs = func
            .basic_blocks
            .into_iter()
            .map(|block| {
                let instrs = block.instrs.iter().map(lower).collect::<Vec<_>>();
                (block.name.to_string(), instrs)
            })
            .collect::<Vec<_>>();

        for line in lower_function(&func.name, &all_instrs, "sm_75") {
            ptx.push_str(&line);
            ptx.push('\n');
        }
    }

    assert!(check_single_entry(&ptx), "multiple .entry sections");
    assert!(check_single_header(&ptx), "redundant header declarations");
    assert!(
        check_register_collisions(&ptx),
        "type conflict in register declarations"
    );
    assert!(
        check_terminal_instruction(&ptx),
        "missing ret or bra at function end"
    );
}
