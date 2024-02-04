use crate::memory::HALFWORD;

use crate::instructions::{types::itype::IType, Instruction};

pub const LH: Instruction = Instruction {
    opcode: 0x3 | 0x1 << 12,
    execute: |machine, word| {
        let i = IType::parse(word);

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), HALFWORD) as i16 as i64
                as u64;
    },
};
