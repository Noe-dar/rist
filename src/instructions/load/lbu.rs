use crate::memory::BYTE;

use crate::instructions::{types::itype::IType, Instruction};

pub const LBU: Instruction = Instruction {
    opcode: 0x3 | 0x4 << 12,
    execute: |machine, word| {
        let i = IType::parse(word);

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), BYTE);
    },
};
