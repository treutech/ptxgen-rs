use ir_model::Instruction;

pub fn lower_to_ptx(instrs: &[Instruction]) -> Vec<String> {
    instrs.iter().map(to_ptx).collect()
}

fn to_ptx(instr: &Instruction) -> String {
    match instr {
        Instruction::FMul { dst, lhs, rhs } => {
            format!(
                "    fmul.f32 {}, {}, {};",
                clean_operand(dst),
                clean_operand(lhs),
                clean_operand(rhs)
            )
        }
        Instruction::FAdd { dst, lhs, rhs } => {
            format!(
                "    fadd.f32 {}, {}, {};",
                clean_operand(dst),
                clean_operand(lhs),
                clean_operand(rhs)
            )
        }
        Instruction::Load { dst, src } => {
            format!(
                "    ld.global.f32 {}, {};",
                clean_operand(dst),
                clean_operand(src)
            )
        }
        Instruction::Store { dst, value } => {
            format!(
                "    st.global.f32 {}, {};",
                clean_operand(dst),
                clean_operand(value)
            )
        }
        Instruction::Add { dst, lhs, rhs } => {
            format!(
                "    add.s32 {}, {}, {};",
                clean_operand(dst),
                clean_operand(lhs),
                clean_operand(rhs)
            )
        }
        Instruction::ICmp { dst, lhs, rhs, .. } => {
            format!(
                "    setp.lt.s32 {}, {}, {};",
                clean_operand(dst),
                clean_operand(lhs),
                clean_operand(rhs)
            )
        }
        Instruction::Alloca { dst, .. } => format!("    // alloca for {}", clean_operand(dst)),
        Instruction::GetElementPtr { dst, base, index } => {
            format!(
                "    // gep: {} = {}[{}]",
                clean_operand(dst),
                clean_operand(base),
                clean_operand(index)
            )
        }
        Instruction::Unhandled(s) => format!("    // unhandled: {}", s),
        _ => String::from("    // unsupported instruction"),
    }
}

fn clean_operand(op: &str) -> &str {
    if let Some(pos) = op.rfind(' ') {
        &op[pos + 1..]
    } else {
        op
    }
}
