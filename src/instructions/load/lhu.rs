use crate::memory::HALFWORD;

use crate::instructions::{types::itype::IType, Instruction};

pub const LHU: Instruction = Instruction {
    opcode: 0x3 | 0x5 << 12,
    execute: |machine, word| {
        let i = IType::parse(word);

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), HALFWORD);
    },
};
