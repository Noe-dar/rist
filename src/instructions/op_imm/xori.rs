use crate::instructions::{types::itype::IType, Instruction};

pub const XORI: Instruction = Instruction {
    opcode: 0x4013,
    execute: |machine, word| {
        let i = IType::parse(word);
        machine.xregisters[i.rd] = machine.xregisters[i.rs1] ^ i.imm;
    },
};
