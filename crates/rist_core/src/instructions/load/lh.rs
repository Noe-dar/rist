use rist_macros::{bake_mask, bake_opcode};

use crate::debug;
use crate::memory::HALFWORD;

use crate::instructions::{types::itype::IType, Instruction};

pub const LH: Instruction = Instruction {
    opcode: bake_opcode!(lh),
    mask: bake_mask!(lh),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: lh reg(i.rd), reg(i.rs1), imm(i.imm));

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), HALFWORD) as i16 as i64
                as u64;
    },
};
