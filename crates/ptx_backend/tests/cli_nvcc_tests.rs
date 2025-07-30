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

use std::process::Command;
use std::fs;

fn run_nvcc_smoke_test(ll_path: &str) {
    let output = Command::new("cargo")
        .args(["run", "--bin", "ptx-backend", "--", ll_path, "--emit"])
        .output()
        .expect("Failed to run cargo");

    assert!(
        output.status.success(),
        "PTX generation failed for {}: {}",
        ll_path,
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(fs::metadata("out.ptx").is_ok(), "Missing out.ptx");

    let status = Command::new("nvcc")
        .args(["-arch=sm_75", "-ptxas-options=-v", "-c", "out.ptx", "-o", "out.o"])
        .status()
        .expect("Failed to run nvcc");

    assert!(
        status.success(),
        "nvcc failed for {}. Check out.ptx syntax.",
        ll_path
    );

    assert!(
        fs::metadata("out.o").map(|m| m.len()).unwrap_or(0) > 0,
        "Output file out.o is empty or missing"
    );
}

#[test]
fn test_saxpy_ptx_compiles() {
    run_nvcc_smoke_test("examples/saxpy.ll");
}

#[test]
fn test_add_ptx_compiles() {
    run_nvcc_smoke_test("examples/add.ll");
}

#[test]
fn test_dot_ptx_compiles() {
    run_nvcc_smoke_test("examples/dot.ll");
}
