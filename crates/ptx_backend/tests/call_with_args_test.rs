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
use regex::Regex;

const CALL_ARGS_LL: &str = include_str!("inputs/call_with_args.ll");

#[test]
fn test_call_with_args_translation() {
    let ptx = compile_llvm_to_ptx(CALL_ARGS_LL).expect("Compilation failed");

    println!("===== PTX Output =====\n{ptx}\n=======================");

    // Validaciones por string directo
    assert!(ptx.contains(".param .s32 arg0;"));
    assert!(ptx.contains(".param .f32 arg1;"));
    assert!(ptx.contains("call foo, (arg0, arg1);"));

    // Validaciones por patrón
    let re_arg0 = Regex::new(r"st\.param\.\w+\s+\[arg0],\s+%[a-zA-Z0-9_]+;").unwrap();
    let re_arg1 = Regex::new(r"st\.param\.\w+\s+\[arg1],\s+%[a-zA-Z0-9_]+;").unwrap();

    assert!(
        re_arg0.is_match(&ptx),
        "Expected st.param.b32 to arg0 from some register"
    );

    assert!(
        re_arg1.is_match(&ptx),
        "Expected st.param.b32 to arg1 from some register"
    );
}
