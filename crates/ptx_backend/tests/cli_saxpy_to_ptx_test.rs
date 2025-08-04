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
use std::path::PathBuf;

#[test]
fn test_cli_saxpy_to_ptx() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("inputs")
        .join("saxpy.ll");

    assert!(path.exists(), "LLVM input file not found: {:?}", path);

    let out_path = PathBuf::from("out.ptx");
    if out_path.exists() {
        fs::remove_file(&out_path).unwrap(); // limpia archivo viejo
    }

    Command::cargo_bin("ptx-backend")
        .expect("binary not built")
        .arg(&path)
        .arg("--emit")
        .assert()
        .success();

    assert!(out_path.exists(), "Expected out.ptx to be created");

    let out_str = fs::read_to_string(out_path).unwrap();
    assert!(
        out_str.contains(".entry saxpy"),
        "Expected PTX to contain `.entry saxpy`, got:\n{}",
        out_str
    );
}
