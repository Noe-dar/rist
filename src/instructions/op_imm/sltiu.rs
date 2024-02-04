use crate::instructions::{types::itype::IType, Instruction};

pub const SLTIU: Instruction = Instruction {
    opcode: 0x3013,
    execute: |machine, word| {
        let i = IType::parse(word);
        machine.xregisters[i.rd] = if machine.xregisters[i.rs1] < i.imm { 1 } else { 0 }
    },
};
