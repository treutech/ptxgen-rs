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

use ptx_backend::compile_llvm_to_ptx;

const CALL_LL: &str = include_str!("inputs/call.ll");

#[test]
fn test_call_translation() {
    let ptx = compile_llvm_to_ptx(CALL_LL).expect("Compilation failed");

    assert!(ptx.contains("call foo, ();"), "Missing call to foo");
    assert!(ptx.contains(".entry main"), "Missing main entry");
    assert!(ptx.contains(".entry foo"), "Missing foo entry");
}
