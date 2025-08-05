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

pub mod ptx_type;
pub mod utils;
pub mod type_map;

use crate::ptx_type::PTXType;
use crate::utils::{clean_operand, get_register_type};
use ir_model::Instruction;
use crate::type_map::{TypeMap, declare_registers_from_typemap};


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

    let mut type_map = TypeMap::new();
    for instr in &flat_instrs {
        for operand in instr.used_operands() {
            if let Some(ty_str) = get_register_type(instr, &operand) {
                let ptx_ty = PTXType::from_str(ty_str);
                type_map.insert(&clean_operand(&operand), ptx_ty);
            }
        }
    }
    for line in declare_registers_from_typemap(&type_map) {
        output.push(line);
    }

    for (block_name, instrs) in all_instrs {
        if instrs.is_empty() {
            continue;
        }
        output.push(format!("{}:", clean_operand(block_name)));
        for instr in instrs {
            let line = to_ptx(instr, &type_map);
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

pub fn to_ptx(instr: &Instruction, type_map: &TypeMap) -> String {
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

    fn extract_arg_name(arg: &str) -> String {
        if arg.starts_with("i32 ") || arg.starts_with("f32 ") {
            arg.split_whitespace().last().unwrap_or(arg).to_string()
        } else {
            arg.to_string()
        }
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
        ICmp {
            dst, lhs, rhs, op, ..
        } => {
            let pred = match op.as_str() {
                "EQ" => "eq",
                "NE" => "ne",
                "UGT" | "SGT" => "gt",
                "UGE" | "SGE" => "ge",
                "ULT" | "SLT" => "lt",
                "ULE" | "SLE" => "le",
                _ => {
                    return format!("// unsupported icmp predicate: {}", op);
                }
            };
            format!(
                "setp.{}.s32 {}, {}, {};",
                pred,
                reg(dst),
                reg(lhs),
                reg(rhs)
            )
        }
        FCmp {
            dst, lhs, rhs, op, ..
        } => {
            let pred = match op.as_str() {
                "OEQ" | "UEQ" => "eq",
                "ONE" | "UNE" => "ne",
                "OGT" | "UGT" => "gt",
                "OGE" | "UGE" => "ge",
                "OLT" | "ULT" => "lt",
                "OLE" | "ULE" => "le",
                _ => "lt", // fallback
            };
            format!(
                "setp.{}.f32 {}, {}, {};",
                pred,
                reg(dst),
                reg(lhs),
                reg(rhs)
            )
        }
        Load { dst, src, .. } => {
            let ty = type_map
                .get(&clean_operand(dst))
                .unwrap_or(&PTXType::S32)
                .as_str();

            let space = "global";
            format!("ld.{space}.{ty} {}, [{}];", reg(dst), clean_operand(src))
        }
        Store { dst, value, .. } => {
            let ty = type_map
                .get(&clean_operand(value))
                .unwrap_or(&PTXType::S32)
                .as_str();

            let space = "global";
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
        CondBr {
            cond,
            then_target,
            else_target,
            ..
        } => {
            format!(
                "@{cond} bra {then};\n    bra {els};",
                cond = reg(cond),
                then = then_target,
                els = else_target
            )
        }
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
            let ty = type_map
                .get(&clean_operand(dst))
                .unwrap_or(&PTXType::S32)
                .as_str();
            format!("// phi.{ty} {} <- {:?}", reg(dst), incoming)
        }
        Alloca { .. } => String::new(),
        Select {
            dst,
            cond,
            val_true,
            val_false,
            ..
        } => {
            let ty = type_map
                .get(&clean_operand(dst))
                .unwrap_or(&PTXType::S32)
                .as_str();
            format!(
                "selp.{ty} {}, {}, {}, {};",
                reg(dst),
                reg(val_true),
                reg(val_false),
                reg(cond)
            )
        }
        Bitcast { dst, src, .. } => {
            format!("mov.b32 {}, {};", reg(dst), reg(src))
        }
        ZExt { dst, src, .. } => {
            format!("cvt.u32.u8 {}, {};", reg(dst), reg(src))
        }
        Trunc { dst, src, .. } => {
            format!("cvt.u8.u32 {}, {};", reg(dst), reg(src))
        }
        Call {
            callee, args, ret, ..
        } => {
            let mut ptx = String::new();

            if let Some(retvar) = ret {
                let ty = type_map
                    .get(&clean_operand(retvar))
                    .unwrap_or(&PTXType::S32)
                    .as_str();
                ptx.push_str(&format!("\t.param .{ty} retval_{};\n", retvar));
            }

            for (i, arg) in args.iter().enumerate() {
                let arg_val = extract_arg_name(arg);
                let ty = type_map
                    .get(&clean_operand(&arg_val))
                    .unwrap_or(&PTXType::S32)
                    .as_str();
                ptx.push_str(&format!("\t.param .{ty} arg{i};\n"));
            }

            for (i, arg) in args.iter().enumerate() {
                let arg_val = extract_arg_name(arg);
                let reg_name = reg(&arg_val);
                let ty = type_map
                    .get(&clean_operand(&arg_val))
                    .unwrap_or(&PTXType::S32)
                    .as_str();

                ptx.push_str(&format!("\tst.param.{ty} [arg{i}], {reg_name};\n"));
            }

            let arg_params = (0..args.len())
                .map(|i| format!("arg{i}"))
                .collect::<Vec<_>>()
                .join(", ");

            if let Some(retvar) = ret {
                ptx.push_str(&format!(
                    "\tcall (retval_{retvar}) {}, ({});\n",
                    clean_operand(callee),
                    arg_params
                ));
                let ty = type_map
                    .get(&clean_operand(retvar))
                    .unwrap_or(&PTXType::S32)
                    .as_str();
                ptx.push_str(&format!(
                    "\tld.param.{ty} {}, [retval_{}];\n",
                    reg(retvar),
                    retvar
                ));
            } else {
                ptx.push_str(&format!(
                    "\tcall {}, ({});\n",
                    clean_operand(callee),
                    arg_params
                ));
            }

            ptx
        }

        Unhandled { text, .. } => format!("// unhandled: {}", text),
    }
}

use anyhow::Result;
use llvm_ir::Module;
use llvm_parser::parse_llvm_ir_from_str;

pub fn compile_llvm_to_ptx(ir_code: &str) -> Result<String> {
    let module: Module = parse_llvm_ir_from_str(ir_code)?;
    let mut ptx_lines = vec![];

    for func in &module.functions {
        let blocks = llvm_parser::lower(func)?;
        let kernel_name = &func.name;

        // Count no-supported instructions per function
        let flat_instrs: Vec<&Instruction> = blocks
            .iter()
            .flat_map(|(_, instrs)| instrs.iter())
            .collect();

        let unhandled_count = flat_instrs
            .iter()
            .filter(|i| matches!(i, Instruction::Unhandled { .. }))
            .count();

        if unhandled_count > 0 {
            eprintln!(
                "Warning: {unhandled_count} unhandled instruction(s) in function `{}`",
                kernel_name
            );
        }

        let func_lines = lower_function(kernel_name, &blocks, "sm_75");
        ptx_lines.extend(func_lines);
        ptx_lines.push(String::new());
    }

    Ok(ptx_lines.join("\n"))
}
