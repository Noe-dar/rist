use crate::instructions::{types::itype::IType, Instruction};

pub const ADDI: Instruction = Instruction {
    opcode: 0x13,
    execute: |machine, word| {
        let i = IType::parse(word);
        machine.xregisters[i.rd] = machine.xregisters[i.rs1].wrapping_add(i.imm);
    },
};
