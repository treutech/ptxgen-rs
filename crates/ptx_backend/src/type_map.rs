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

use std::collections::HashMap;
use crate::ptx_type::PTXType;

#[derive(Debug, Default)]
pub struct TypeMap {
    types: HashMap<String, PTXType>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, var: &str, ty: PTXType) {
        self.types.insert(var.to_string(), ty);
    }

    pub fn get(&self, var: &str) -> Option<&PTXType> {
        self.types.get(var)
    }

    pub fn dominant_type(types: &[PTXType]) -> Option<PTXType> {
        use PTXType::*;
        if types.is_empty() {
            return None;
        }

        if types.iter().all(|t| *t == F32) {
            Some(F32)
        } else if types.iter().all(|t| *t == S32) {
            Some(S32)
        } else if types.iter().all(|t| *t == Pred) {
            Some(Pred)
        } else if types.contains(&F32) {
            Some(F32) // prefer float in mixed
        } else if types.contains(&S32) {
            Some(S32) // fallback
        } else {
            None
        }
    }

    pub fn all(&self) -> &HashMap<String, PTXType> {
        &self.types
    }
}


pub fn declare_registers_from_typemap(map: &TypeMap) -> Vec<String> {
    use std::collections::BTreeMap;
    let mut reg_by_type: BTreeMap<PTXType, Vec<&str>> = BTreeMap::new();

    for (reg, ty) in map.all() {
        reg_by_type.entry(*ty).or_default().push(reg);
    }

    let mut lines = vec![];
    for (ty, mut regs) in reg_by_type {
        regs.sort(); // 🔥 Aquí imponemos orden alfabético en los nombres de registros
        let regs_str = regs.iter().map(|r| format!("%{}", r)).collect::<Vec<_>>().join(", ");
        lines.push(format!(".reg {} {};", ty.as_str(), regs_str));
    }

    lines
}
