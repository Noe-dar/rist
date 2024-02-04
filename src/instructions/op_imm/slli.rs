use crate::instructions::{types::itype::IType, Instruction};

pub const SLLI: Instruction = Instruction {
    opcode: 0x13 | 0x1 << 12,
    execute: |machine, word| {
        let i = IType::parse(word);
        let shamt: u64 = i.imm & 0xF;
        machine.xregisters[i.rd] = (machine.xregisters[i.rs1] << shamt) as i32 as i64 as u64
    },
};
