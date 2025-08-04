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

pub mod convert;
pub mod parse_module;

use anyhow::Result;
use llvm_ir::{Function, Module};
use ir_model::Instruction;

pub fn parse_llvm_ir_from_str(ir: &str) -> Result<Module> {
    let module = Module::from_ir_str(ir).map_err(anyhow::Error::msg)?;
    Ok(module)
}

pub fn lower(func: &Function) -> Result<Vec<(String, Vec<Instruction>)>> {
    let mut blocks = vec![];
    for block in &func.basic_blocks {
        let mut instrs = vec![];

        for instr in &block.instrs {
            instrs.push(convert::lower(&func.name, instr));
        }

        // 👇 Esta parte es clave: también baja el terminator
        instrs.push(convert::lower_terminator(&func.name, &block.term));

        blocks.push((block.name.to_string(), instrs));
    }
    Ok(blocks)
}

