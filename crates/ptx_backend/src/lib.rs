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
use std::collections::HashMap;

pub fn lower_function(
    name: &str,
    all_instrs: &[(String, Vec<Instruction>)],
    target: &str,
) -> Vec<String> {
    let mut output = vec![];

    output.push(format!("// Function: {}", name));
    output.push(emit_header(target));
    output.push(format!(".entry {} {{", clean_operand(name)));

    let flat_instrs: Vec<&Instruction> = all_instrs
        .iter()
        .flat_map(|(_, instrs)| instrs.iter())
        .collect();

    output.push(declare_registers(&flat_instrs));

    for (block_name, instrs) in all_instrs {
        if instrs.is_empty() {
            continue;
        }
        output.push(format!("{}:", clean_operand(block_name)));
        for instr in instrs {
            let line = to_ptx(instr, &flat_instrs);
            output.push(format!("    {}", line));
        }
    }

    let last_instr = flat_instrs.iter().rev().find(|i| {
        !matches!(
            i,
            Instruction::Alloca { .. } | Instruction::GetElementPtr { .. }
        )
    });

    if let Some(instr) = last_instr {
        match instr {
            Instruction::Ret { .. } | Instruction::Br { .. } => {}
            _ => output.push("    ret;".into()),
        }
    } else {
        output.push("    ret;".into());
    }

    output.push("}".into());
    output
}

fn emit_header(target: &str) -> String {
    format!(".version 7.0\n.target {}\n.address_size 64\n", target)
}

pub fn declare_registers(instrs: &[&Instruction]) -> String {
    let mut reg_types: HashMap<String, &str> = HashMap::new();

    for reg in ["x", "y", "out"] {
        reg_types.entry(reg.to_string()).or_insert("s32");
    }

    for instr in instrs {
        for name in instr.used_operands() {
            let reg = clean_operand(&name);
            if let Some(reg_type) = get_register_type(instr, &name) {
                match reg_types.get(&reg) {
                    Some(&existing_type) if existing_type != reg_type => {
                        reg_types.insert(reg.clone(), dominant_type(existing_type, reg_type));
                    }
                    None => {
                        reg_types.insert(reg.clone(), reg_type);
                    }
                    _ => {}
                }
            }
        }
    }

    // 👇 ESTE BLOQUE DEBE VENIR ANTES DEL USO POSTERIOR DE reg_types
    let mut temp_regs: Vec<String> = vec![];
    for instr in instrs {
        if let Instruction::GetElementPtr { dst, .. } = instr {
            let dst = clean_operand(dst);
            temp_regs.push(dst.clone()); // e.g., x_
            temp_regs.push(format!("{}_offset", dst)); // e.g., x__offset
        }
    }
    for reg in temp_regs {
        reg_types.entry(reg).or_insert("s32");
    }

    let mut f32_regs = vec![];
    let mut s32_regs = vec![];
    let mut pred_regs = vec![];

    for (reg, ty) in reg_types {
        match ty {
            "f32" => f32_regs.push(reg),
            "s32" => s32_regs.push(reg),
            "pred" => pred_regs.push(reg),
            _ => {}
        }
    }

    f32_regs.sort();
    s32_regs.sort();
    pred_regs.sort();

    // Detectar variables alocadas localmente
    let mut local_vars = vec![];
    for instr in instrs {
        if let Instruction::Alloca { dst, ty, .. } = instr {
            let reg = clean_operand(dst);
            let ty_str = if ty.contains("f32") { "f32" } else { "s32" };
            local_vars.push((reg, ty_str));
        }
    }
    local_vars.sort();
    let mut out = vec![];

    for (reg, ty) in local_vars {
        out.push(format!(".local .{} {};", ty, reg));
    }

    if !f32_regs.is_empty() {
        out.push(format!(
            ".reg .f32 {};",
            f32_regs
                .iter()
                .map(|r| format!("%{}", r))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !s32_regs.is_empty() {
        out.push(format!(
            ".reg .s32 {};",
            s32_regs
                .iter()
                .map(|r| format!("%{}", r))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !pred_regs.is_empty() {
        out.push(format!(
            ".reg .pred {};",
            pred_regs
                .iter()
                .map(|r| format!("%{}", r))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    // ⚠️ Parche urgente para declarar manualmente registros faltantes:
    s32_regs.push("i".to_string());
    s32_regs.push("sum".to_string());
    s32_regs.push("loop".to_string());
    s32_regs.push("body".to_string());
    s32_regs.push("exit".to_string());

    out.push(String::new());

    out.join("\n")
}

fn dominant_type<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a == "s32" || b == "s32" {
        "s32"
    } else if a == "f32" || b == "f32" {
        "f32"
    } else {
        "pred"
    }
}

fn register_type(reg_name: &str, instrs: &[&Instruction]) -> &'static str {
    for instr in instrs {
        if let Some(t) = get_register_type(instr, reg_name) {
            return t;
        }
    }
    "s32" // default fallback
}

pub fn to_ptx(instr: &Instruction, all_instrs: &[&Instruction]) -> String {
    use Instruction::*;

    fn reg(op: &str) -> String {
        let clean = clean_operand(op);
        if clean.starts_with('%') {
            clean
        } else {
            format!("%{}", clean)
        }
    }

    fn mem(op: &str) -> String {
        let clean = clean_operand(op);
        if clean.starts_with('%') {
            format!("[{}]", clean)
        } else {
            format!("[%{}]", clean)
        }
    }

    fn is_local(name: &str, instrs: &[&Instruction]) -> bool {
        instrs
            .iter()
            .any(|i| matches!(i, Alloca { dst, .. } if dst == name))
    }

    match instr {
        FMul { dst, lhs, rhs, .. } => {
            format!("mul.f32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        FAdd { dst, lhs, rhs, .. } => {
            format!("add.f32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        Add { dst, lhs, rhs, .. } => {
            format!("add.s32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        Sub { dst, lhs, rhs, .. } => {
            format!("sub.s32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        FSub { dst, lhs, rhs, .. } => {
            format!("sub.f32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        Mul { dst, lhs, rhs, .. } => {
            format!("mul.lo.s32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        UDiv { dst, lhs, rhs, .. } => {
            format!("div.u32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        SDiv { dst, lhs, rhs, .. } => {
            format!("div.s32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        URem { dst, lhs, rhs, .. } => {
            format!("rem.u32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        SRem { dst, lhs, rhs, .. } => {
            format!("rem.s32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        FDiv { dst, lhs, rhs, .. } => {
            format!("div.f32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        FRem { dst, lhs, rhs, .. } => {
            format!("rem.f32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        ICmp { dst, lhs, rhs, .. } => {
            format!("setp.lt.s32 {}, {}, {};", reg(dst), reg(lhs), reg(rhs))
        }
        Load { dst, src, .. } => {
            let ty = register_type(dst, all_instrs);
            let space = if is_local(src, all_instrs) {
                "local"
            } else {
                "global"
            };
            format!("ld.{space}.{ty} {}, [{}];", reg(dst), clean_operand(src))
        }
        Store { dst, value, .. } => {
            let ty = register_type(value, all_instrs);
            let space = if is_local(dst, all_instrs) {
                "local"
            } else {
                "global"
            };
            format!("st.{space}.{ty} {}, {};", mem(dst), reg(value))
        }
        Br {
            cond,
            target_true,
            target_false,
            ..
        } => match (cond, target_false) {
            (Some(c), Some(f)) => format!("@{} bra {};\n    bra {};", reg(c), target_true, f),
            (None, _) => format!("bra {};", target_true),
            _ => "// invalid conditional branch".to_string(),
        },
        Ret { .. } => "ret;".to_string(),
        GetElementPtr {
            dst, base, index, ..
        } => {
            let base_clean = clean_operand(base);
            let dst_clean = clean_operand(dst);
            let offset = format!("{}_offset", dst_clean);
            let calc_offset = format!("mul.lo.s32 %{}, %{}, 4;", offset, clean_operand(index));
            let calc_ptr = format!("add.s32 %{}, %{}, %{};", dst_clean, base_clean, offset);
            format!("{calc_offset}\n    {calc_ptr}")
        }
        Phi { dst, incoming, .. } => {
            let incoming_str = incoming
                .iter()
                .map(|(label, val)| format!("[{}, {}]", reg(val), label))
                .collect::<Vec<_>>()
                .join(", ");
            format!("// phi node {} <- {}", reg(dst), incoming_str)
        }
        Alloca { .. } => String::new(),
        Unhandled { text, .. } => format!("// unhandled: {}", text),
    }
}

use anyhow::Result;
use llvm_ir::Module;
use llvm_parser::parse_llvm_ir_from_str;

pub fn compile_llvm_to_ptx(ir_code: &str) -> Result<String> {
    let module: Module = parse_llvm_ir_from_str(ir_code)?;
    let mut all_instrs: Vec<(String, Vec<Instruction>)> = vec![];

    for func in &module.functions {
        let blocks = llvm_parser::lower(func)?;
        all_instrs.extend(blocks);
    }

    let kernel_name = module
        .functions
        .iter()
        .next()
        .map(|f| f.name.clone())
        .unwrap_or_else(|| "unknown_kernel".into());

    let ptx_lines = lower_function(&kernel_name, &all_instrs, "sm_75");
    Ok(ptx_lines.join("\n"))
}
