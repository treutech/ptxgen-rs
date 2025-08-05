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
use std::process::Command;

fn run_nvcc_smoke_test(name: &str) {

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("inputs")
        .join(name);
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "ptx-backend", "--", path.to_str().unwrap(), "--emit"])
        .output()
        .expect("Failed to run cargo");

    assert!(
        output.status.success(),
        "PTX generation failed for {:?}: {}",
        path,
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(fs::metadata("out.ptx").is_ok(), "Missing out.ptx");

    let status = Command::new("nvcc")
        .args(["-arch=sm_75", "-cubin", "out.ptx", "-o", "out.cubin"])
        .status()
        .expect("Failed to run nvcc");

    assert!(
        status.success(),
        "nvcc failed for {}. Check out.ptx syntax.",
        path.to_str().unwrap()
    );

    assert!(
        fs::metadata("out.cubin").map(|m| m.len()).unwrap_or(0) > 0,
        "Output file out.cubin is empty or missing"
    );
}

#[test]
fn test_minimal_ret_ptx_compiles() {
    run_nvcc_smoke_test("minimal_ret.ll");
}


#[test]
fn test_dot_ptx_compiles() {
    run_nvcc_smoke_test("dot.ll");
}

/*
 TODO

#[test]
fn test_add_ptx_compiles() {
    run_nvcc_smoke_test("add.ll");
}

#[test]
fn test_phi_ptx_generates() {
    run_nvcc_smoke_test("phi.ll");
}

#[test]
fn test_saxpy_ptx_compiles() {
    run_nvcc_smoke_test("saxpy.ll");
}
*/
