use ir_model::Instruction;
use llvm_ir::instruction::Instruction as LlvmInst;

pub fn lower(instr: &LlvmInst) -> Instruction {
    use LlvmInst::*;
    match instr {
        FMul(f) => Instruction::FMul {
            dst: f.dest.to_string(),
            lhs: f.operand0.to_string(),
            rhs: f.operand1.to_string(),
        },
        FAdd(f) => Instruction::FAdd {
            dst: f.dest.to_string(),
            lhs: f.operand0.to_string(),
            rhs: f.operand1.to_string(),
        },
        Load(l) => Instruction::Load {
            dst: l.dest.to_string(),
            src: l.address.to_string(),
        },
        Store(s) => Instruction::Store {
            dst: s.address.to_string(),
            value: s.value.to_string(),
        },
        Alloca(a) => Instruction::Alloca {
            dst: a.dest.to_string(),
            ty: format!("{:?}", a.allocated_type),
            align: a.alignment,
        },
        ICmp(cmp) => Instruction::ICmp {
            dst: cmp.dest.to_string(),
            op: format!("{:?}", cmp.predicate),
            lhs: cmp.operand0.to_string(),
            rhs: cmp.operand1.to_string(),
        },
        Add(add) => Instruction::Add {
            dst: add.dest.to_string(),
            lhs: add.operand0.to_string(),
            rhs: add.operand1.to_string(),
        },
        GetElementPtr(gep) => Instruction::GetElementPtr {
            dst: gep.dest.to_string(),
            base: gep.address.to_string(),
            index: gep
                .indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        },
        _ => Instruction::Unhandled(format!("{:?}", instr)),
    }
}
