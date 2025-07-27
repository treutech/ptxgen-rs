
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Instruction {
    Load { dst: String, src: String },
    Store { dst: String, value: String },
    Add { dst: String, lhs: String, rhs: String },
    FAdd { dst: String, lhs: String, rhs: String },
    FMul { dst: String, lhs: String, rhs: String },
    ICmp { dst: String, lhs: String, rhs: String, op: String },
    GetElementPtr { dst: String, base: String, index: String },
    Alloca { dst: String, ty: String, align: u32 },
    Br { cond: Option<String>, target_true: String, target_false: Option<String> },
    Ret,
    Unhandled(String),
}

