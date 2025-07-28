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

mod utils;
use crate::utils::{clean_operand, get_register_type};
use ir_model::Instruction;
use std::collections::HashSet;

pub fn lower_function(name: &str, all_instrs: &[(String, Vec<Instruction>)]) -> Vec<String> {
    let mut output = vec![];

    output.push(format!("// Function: {}", name));
    output.push(emit_header());

    let flat_instrs: Vec<&Instruction> = all_instrs
        .iter()
        .flat_map(|(_, instrs)| instrs.iter())
        .collect();

    output.push(declare_registers(&flat_instrs));
    output.push(format!(".entry {} {{", name));

    for (block_name, instrs) in all_instrs {
        output.push(format!("    // Block: {}", block_name));
        for instr in instrs {
            let line = to_ptx(instr);
            output.push(format!("    {}", line));
        }
    }

    output.push("}".into());
    output
}

pub fn lower_to_ptx(instrs: &[&Instruction]) -> Vec<String> {
    let mut output = vec![];

    output.push("// Function: saxpy".into());
    output.push(emit_header());
    output.push(declare_registers(instrs));
    output.push(".entry saxpy {".into());

    for instr in instrs {
        output.push(to_ptx(instr));
    }

    output.push("}".into());
    output
}

fn emit_header() -> String {
    [
        ".version 7.0",
        ".target sm_75",
        ".address_size 64",
        "".into(),
    ]
    .join("\n")
}

pub fn declare_registers(instrs: &[&Instruction]) -> String {
    let mut f32_regs = HashSet::new();
    let mut s32_regs = HashSet::new();
    let mut pred_regs = HashSet::new();

    for instr in instrs {
        for name in instr.used_operands() {
            let reg = clean_operand(&name);
            match get_register_type(instr, &name) {
                Some("f32") => {
                    f32_regs.insert(reg.to_string());
                }
                Some("s32") => {
                    s32_regs.insert(reg.to_string());
                }
                Some("pred") => {
                    pred_regs.insert(reg.to_string());
                }
                _ => {}
            }
        }
    }

    let mut out = vec![];
    if !f32_regs.is_empty() {
        out.push(format!(
            ".reg .f32 {};",
            f32_regs
                .iter()
                .map(|r: &String| format!("%{}", r))
                .collect::<Vec<String>>()
                .join(", ")
        ));
    }
    if !s32_regs.is_empty() {
        out.push(format!(
            ".reg .s32 {};",
            s32_regs
                .iter()
                .map(|r: &String| format!("%{}", r))
                .collect::<Vec<String>>()
                .join(", ")
        ));
    }
    if !pred_regs.is_empty() {
        out.push(format!(
            ".reg .pred {};",
            pred_regs
                .iter()
                .map(|r: &String| format!("%{}", r))
                .collect::<Vec<String>>()
                .join(", ")
        ));
    }
    out.push(String::new()); // for newline
    out.join("\n")
}

pub fn to_ptx(instr: &Instruction) -> String {
    match instr {
        Instruction::FMul { dst, lhs, rhs } => {
            format!(
                "    fmul.f32 {}, {}, {};",
                clean_operand(dst),
                clean_operand(lhs),
                clean_operand(rhs)
            )
        }
        Instruction::FAdd { dst, lhs, rhs } => {
            format!(
                "    fadd.f32 {}, {}, {};",
                clean_operand(dst),
                clean_operand(lhs),
                clean_operand(rhs)
            )
        }
        Instruction::Load { dst, src } => {
            format!(
                "    ld.global.f32 {}, {};",
                clean_operand(dst),
                clean_operand(src)
            )
        }
        Instruction::Store { dst, value } => {
            format!(
                "    st.global.f32 {}, {};",
                clean_operand(dst),
                clean_operand(value)
            )
        }
        Instruction::Add { dst, lhs, rhs } => {
            format!(
                "    add.s32 {}, {}, {};",
                clean_operand(dst),
                clean_operand(lhs),
                clean_operand(rhs)
            )
        }
        Instruction::ICmp { dst, lhs, rhs, .. } => {
            format!(
                "    setp.lt.s32 {}, {}, {};",
                clean_operand(dst),
                clean_operand(lhs),
                clean_operand(rhs)
            )
        }
        Instruction::Alloca { dst, .. } => {
            format!("    // local stack allocation: {}", clean_operand(dst))
        }
        Instruction::GetElementPtr { dst, base, index } => {
            format!(
                "    // address calc: {} = {}[{}]",
                clean_operand(dst),
                clean_operand(base),
                clean_operand(index)
            )
        }
        Instruction::Unhandled(s) => format!("    // unhandled: {}", s),
        _ => String::from("    // unsupported instruction"),
    }
}
