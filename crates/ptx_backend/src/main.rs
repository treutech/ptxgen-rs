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
use llvm_parser::parse_module::parse_module;
use ptx_backend::lower_to_ptx;
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

    for func in module.functions {
        writeln!(output, "// Function: {}", func.name).unwrap();

        let mut instrs = Vec::new();
        for block in func.basic_blocks {
            // Optional comment per block
            writeln!(output, "// Block: {}", block.name).unwrap();
            instrs.extend(block.instrs.iter().map(llvm_parser::convert::lower));
        }

        for line in lower_to_ptx(&instrs) {
            writeln!(output, "{}", line).unwrap();
        }
    }

}
