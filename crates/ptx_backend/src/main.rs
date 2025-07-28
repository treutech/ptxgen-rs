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

use clap::Parser;
use ir_model::Instruction;
use llvm_parser::parse_module::parse_module;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Parser)]
struct Args {
    input: String,
    #[arg(long)]
    emit: bool,
}

fn main() {
    let args = Args::parse();
    let module = parse_module(&args.input).expect("invalid LLVM IR");

    let mut output: Box<dyn Write> = if args.emit {
        Box::new(BufWriter::new(File::create("out.ptx").unwrap()))
    } else {
        Box::new(std::io::stdout())
    };

    // Emitir encabezado global solo una vez
    writeln!(output, ".version 7.0").unwrap();
    writeln!(output, ".target sm_75").unwrap();
    writeln!(output, ".address_size 64").unwrap();
    writeln!(output).unwrap();

    for func in module.functions {
        writeln!(output, "// Function: {}", func.name).unwrap();

        // Reunir todas las instrucciones de todos los bloques
        let mut instrs = vec![];
        for block in &func.basic_blocks {
            writeln!(output, "// Block: {}", block.name).unwrap();
            instrs.extend(block.instrs.iter().map(llvm_parser::convert::lower));
        }

        // Emitir declaración de registros y encabezado de función
        let instr_refs: Vec<&Instruction> = instrs.iter().collect();
        writeln!(output, "{}", ptx_backend::declare_registers(&instr_refs)).unwrap();
        writeln!(output, ".entry {} {{", func.name).unwrap();

        // Emitir instrucciones
        for instr in &instrs {
            writeln!(output, "{}", ptx_backend::to_ptx(instr)).unwrap();
        }

        writeln!(output, "}}").unwrap(); // cerrar función
        writeln!(output).unwrap();
    }
}
