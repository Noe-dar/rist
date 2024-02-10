use rist_macros::{bake_mask, bake_opcode};

use crate::{
    debug, instructions::{types::stype::SType, Instruction}, memory::BYTE
};

pub const SB: Instruction = Instruction {
    opcode: bake_opcode!(sb),
    mask: bake_mask!(sb),
    execute: |machine, word| {
        let s = SType::parse(word);
        debug!(machine: sb reg(s.rs1), reg(s.rs2), imm(s.imm));

        machine.bus.write(
            machine.xregisters[s.rs1].wrapping_add(s.imm),
            machine.xregisters[s.rs2],
            BYTE,
        )
    },
};
