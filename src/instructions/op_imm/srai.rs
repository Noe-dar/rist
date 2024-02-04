use crate::instructions::{types::itype::IType, Instruction};

pub const SRAI: Instruction = Instruction {
    opcode: u32::MAX,
    execute: |machine, word| {
        let i = IType::parse(word);
        let shamt: u64 = i.imm & 0xF;

        machine.xregisters[i.rd] = machine.xregisters[i.rs1] >> shamt
    },
};
