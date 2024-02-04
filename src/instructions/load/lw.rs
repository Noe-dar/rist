use crate::memory::WORD;

use crate::instructions::{types::itype::IType, Instruction};

pub const LW: Instruction = Instruction {
    opcode: 0x3 | 0x2 << 12,
    execute: |machine, word| {
        let i = IType::parse(word);

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), WORD) as i32 as i64
                as u64;
    },
};
