pub mod convert;

use llvm_ir::Module;
use anyhow::Result;

pub fn parse_llvm_ir_from_str(ir: &str) -> Result<Module> {
    let module = Module::from_ir_str(ir).map_err(anyhow::Error::msg)?;
    Ok(module)
}
