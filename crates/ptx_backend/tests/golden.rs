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

use std::fs;
use std::path::Path;
use ptx_backend::compile_llvm_to_ptx;

fn check_golden(test_name: &str) {

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("inputs")
        .join(format!("{test_name}.ll"));
    
    let input = fs::read_to_string(&path).expect("failed to read .ll");

    let ptx = compile_llvm_to_ptx(&input).expect("compilation failed");

    let golden_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
        .join(format!("{test_name}.ptx"));

    let expected = fs::read_to_string(&golden_path).expect("golden .ptx missing");

    assert_eq!(ptx.trim(), expected.trim(), "Mismatch in {}", test_name);
}

#[test]
fn golden_ret_only() {
    check_golden("ret_only");
}

#[test]
fn golden_add() {
    check_golden("add");
}

#[test]
fn golden_sub() {
    check_golden("sub");
}

#[test]
fn golden_mul() {
    check_golden("mul");
}

#[test]
fn golden_div() {
    check_golden("div");
}

#[test]
fn golden_rem() {
    check_golden("rem");
}

#[test]
fn golden_icmp() {
    check_golden("icmp");
}

#[test]
fn golden_fcmp() {
    check_golden("fcmp");
}

#[test]
fn golden_phi() {
    check_golden("phi");
}

#[test]
fn golden_select() {
    check_golden("select");
}

#[test]
fn golden_bitcast() {
    check_golden("bitcast");
}

#[test]
fn golden_zext() {
    check_golden("zext");
}

#[test]
fn golden_trunc() {
    check_golden("trunc");
}

#[test]
fn golden_branch_cond() {
    check_golden("branch_cond");
}

#[test]
fn golden_branch_uncond() {
    check_golden("branch_uncond");
}


