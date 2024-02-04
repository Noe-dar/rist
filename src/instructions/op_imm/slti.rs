use crate::instructions::{types::itype::IType, Instruction};

pub const SLTI: Instruction = Instruction {
    opcode: 0x2013,
    execute: |machine, word| {
        let i = IType::parse(word);
        machine.xregisters[i.rd] =
            if (machine.xregisters[i.rs1] as i64) < (i.imm as i64) { 1 } else { 0 }
    },
};
