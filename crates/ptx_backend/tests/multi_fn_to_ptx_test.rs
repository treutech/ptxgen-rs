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

const MULTI_FN_LL: &str = r#"
define void @foo() {
entry:
  ret void
}

define void @bar() {
entry:
  ret void
}

define void @baz() {
entry:
  ret void
}
"#;

#[test]
fn test_multiple_functions_generate_multiple_entries() {
    let result = compile_llvm_to_ptx(MULTI_FN_LL).expect("Compilation failed");

    let expected_entries = ["foo", "bar", "baz"];
    for entry in expected_entries {
        let header = format!(".entry {}", entry);
        assert!(
            result.contains(&header),
            "Expected entry '{}' not found in PTX:\n{}",
            entry,
            result
        );
    }

    // Optional sanity check: ensure there are exactly 3 .entry declarations
    let entry_count = result.matches(".entry ").count();
    assert_eq!(
        entry_count, 3,
        "Expected 3 entry points, but found {}.\n{}",
        entry_count, result
    );
}
