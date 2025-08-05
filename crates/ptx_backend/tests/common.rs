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

use ir_model::Instruction;
use ptx_backend::ptx_type::PTXType;
use ptx_backend::type_map::TypeMap;
use ptx_backend::utils::{clean_operand, get_register_type};

pub fn build_typemap(instrs: &[&Instruction]) -> TypeMap {
    let mut type_map = TypeMap::new();

    for instr in instrs {
        for operand in instr.used_operands() {
            if let Some(ty_str) = get_register_type(instr, operand) {
                let ptx_ty = PTXType::from_str(ty_str);
                type_map.insert(&clean_operand(operand), ptx_ty);
            }
        }
    }

    type_map
}