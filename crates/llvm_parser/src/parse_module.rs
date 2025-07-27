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

use anyhow::{Context, Result};
use llvm_ir::Module;
use std::fs;
use std::path::Path;

/// Parse an LLVM IR (.ll) file into an `llvm_ir::Module`
pub fn parse_module<P: AsRef<Path>>(path: P) -> Result<Module> {
    let ll_text = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file: {}", path.as_ref().display()))?;

    Module::from_ir_str(&ll_text)
        .map_err(|e| anyhow::anyhow!(e))
        .with_context(|| format!("Failed to parse LLVM IR in file: {}", path.as_ref().display()))
}
