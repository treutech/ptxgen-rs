use clap::Parser;
use llvm_parser::parse_module::parse_module;
use ptx_backend::lower_to_ptx;
use ir_model::Instruction;
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
        for block in func.basic_blocks {
            writeln!(output, "// Block: {}", block.name).unwrap();
            let instrs: Vec<Instruction> = block.instrs.iter().map(llvm_parser::convert::lower).collect();
            for line in lower_to_ptx(&instrs) {
                writeln!(output, "{}", line).unwrap();
            }
        }
    }
}
