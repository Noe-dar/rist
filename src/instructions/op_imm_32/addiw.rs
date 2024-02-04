use crate::instructions::{types::itype::IType, Instruction};

pub const ADDIW: Instruction = Instruction {
    opcode: 0x1b,
    execute: |machine, word| {
        let i = IType::parse(word);
        machine.xregisters[i.rd] =
            machine.xregisters[i.rs1].wrapping_add(i.imm) as i32 as i64 as u64;
    },
};
