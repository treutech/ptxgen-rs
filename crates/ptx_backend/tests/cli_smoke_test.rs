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

use assert_cmd::Command;
use std::fs;
use std::path::Path;

#[test]
fn test_generated_ptx_is_valid_for_nvcc() {
    let output_path = "out.ptx";

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("inputs")
        .join("minimal_ret.ll");

    Command::cargo_bin("ptx-backend")
        .unwrap()
        .args([path.to_str().unwrap(), "--emit"]) 
        .assert()
        .success();

    assert!(
        Path::new(output_path).exists(),
        "PTX file was not generated"
    );

    let nvcc_status = std::process::Command::new("nvcc")
        .args(["-arch=sm_75", "-cubin", output_path, "-o", "/dev/null"])
        .status()
        .expect("Failed to execute nvcc");

    assert!(
        nvcc_status.success(),
        "nvcc failed to compile the generated PTX"
    );

    fs::remove_file(output_path).ok();
}
