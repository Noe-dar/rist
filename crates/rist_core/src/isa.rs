use constcat::concat_slices;

use crate::instructions::{
    auipc::AUIPC, branch::BRANCH_INSTRUCTIONS, ecall::ECALL, jal::JAL, jalr::JALR, lui::LUI,
    store::sd::SD, Instruction, IMM_32_INSTRUCTIONS, IMM_INSTRUCTIONS, LOAD_INSTRUCTIONS,
    OP_INSTRUCTIONS,
};

pub const RV32_I: &[Instruction] = concat_slices!(
    [Instruction::EMPTY; Instruction]:
    IMM_INSTRUCTIONS, OP_INSTRUCTIONS, BRANCH_INSTRUCTIONS, LOAD_INSTRUCTIONS,
    &[JAL, JALR, LUI, AUIPC, SD, ECALL]
);

pub const RV64_I: &[Instruction] = concat_slices!(
    [Instruction::EMPTY; Instruction]:
    RV32_I, IMM_32_INSTRUCTIONS
);
