use std::env;
use std::fs;
use llvm_ir::Module;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: llvm2ptx <file.ll>");
        std::process::exit(1);
    }

    let file = &args[1];
    let ll_text = fs::read_to_string(file)
        .with_context(|| format!("Failed to read file: {}", file))?;

    let module = Module::from_ir_str(&ll_text)
        .map_err(|e| anyhow::anyhow!("Failed to parse LLVM IR: {}", e))?;

    for func in &module.functions {
        println!("Function: {}", func.name);
        for block in &func.basic_blocks {
            println!("  Basic block: {}", block.name);
            for instr in &block.instrs {
                println!("    {:?}", instr);
            }
        }
    }

    Ok(())
}
