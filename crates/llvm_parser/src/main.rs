use ir_model::Instruction;
use llvm_ir::Module;
use llvm_parser::convert::lower;

use serde_json;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: llvm_parser <file.ll>");

    let ll_text = fs::read_to_string(filename).expect("Failed to read .ll file");
    let module = Module::from_ir_str(&ll_text).expect("Failed to parse LLVM IR");

    for func in &module.functions {
        println!("Function: {}", func.name);
        for block in &func.basic_blocks {
            println!("  Basic block: {}", block.name);
            for instr in &block.instrs {
                let lowered: Instruction = lower(instr);
                println!("{}", serde_json::to_string_pretty(&lowered).unwrap());
            }
        }
    }
}
