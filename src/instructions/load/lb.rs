use crate::memory::BYTE;

use crate::instructions::{types::itype::IType, Instruction};

pub const LB: Instruction = Instruction {
    opcode: 0x3,
    execute: |machine, word| {
        let i = IType::parse(word);

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), BYTE) as i8 as i64
                as u64;
    },
};
