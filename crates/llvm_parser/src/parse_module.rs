use anyhow::{Context, Result};
use llvm_ir::Module;
use std::fs;
use std::path::Path;

/// Parse an LLVM IR (.ll) file into an `llvm_ir::Module`
pub fn parse_module<P: AsRef<Path>>(path: P) -> Result<Module> {
    let ll_text = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file: {}", path.as_ref().display()))?;

    Module::from_ir_str(&ll_text)
        .map_err(|e| anyhow::anyhow!(e))
        .with_context(|| format!("Failed to parse LLVM IR in file: {}", path.as_ref().display()))
}
