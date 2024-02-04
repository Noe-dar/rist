use crate::instructions::{types::itype::IType, Instruction};

pub const ANDI: Instruction = Instruction {
    opcode: 0x7013,
    execute: |machine, word| {
        let i = IType::parse(word);
        machine.xregisters[i.rd] = machine.xregisters[i.rs1] & i.imm
    },
};
