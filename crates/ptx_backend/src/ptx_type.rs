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

// PTX type system for register declaration and operand mapping.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PTXType {
    S32,
    S64,
    F32,
    F64,
    Pred,
    Ptr,
}

// En ptx_type.rs
impl PTXType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "s32" => PTXType::S32,
            "f32" => PTXType::F32,
            "pred" => PTXType::Pred,
            "ptr" => PTXType::Ptr,
            _ => PTXType::S32, // default fallback
        }
    }
}

impl PTXType {
    /// Returns the string representation used in PTX code
    pub fn as_str(&self) -> &'static str {
        match self {
            PTXType::S32 => "s32",
            PTXType::S64 => "s64",
            PTXType::F32 => "f32",
            PTXType::F64 => "f64",
            PTXType::Pred => "pred",
            PTXType::Ptr => "u64", // Punteros tratados como enteros de 64 bits
        }
    }

    /// Given two types used in same register, return the dominant type
    pub fn dominant_with(self, other: PTXType) -> PTXType {
        use PTXType::*;
        match (self, other) {
            (Pred, _) | (_, Pred) => Pred,
            (Ptr, _) | (_, Ptr) => Ptr,
            (F64, _) | (_, F64) => F64,
            (F32, _) | (_, F32) => F32,
            (S64, _) | (_, S64) => S64,
            _ => S32,
        }
    }
}
