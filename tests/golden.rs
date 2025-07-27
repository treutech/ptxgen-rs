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

use ptx_backend::lower_to_ptx;
use llvm_parser::parse_module::parse_module;
use llvm_parser::convert::lower;

#[test]
fn test_saxpy_ptx() {
    let m = parse_module("examples/saxpy.ll").unwrap();
    let mut out = String::new();
    for f in m.functions {
        for b in f.basic_blocks {
            let instrs = b.instrs.iter().map(lower).collect::<Vec<_>>();
            for l in lower_to_ptx(&instrs) {
                out.push_str(&l);
                out.push('\n');
            }
        }
    }
    insta::assert_snapshot!(out);
}
