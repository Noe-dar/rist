use rist_macros::{bake_mask, bake_opcode};

use crate::{
    debug, instructions::{types::stype::SType, Instruction}, memory::WORD
};

pub const SW: Instruction = Instruction {
    opcode: bake_opcode!(sw),
    mask: bake_mask!(sw),
    execute: |machine, word| {
        let s = SType::parse(word);
        debug!(machine: sh reg(s.rs1), reg(s.rs2), imm(s.imm));

        machine.bus.write(
            machine.xregisters[s.rs1].wrapping_add(s.imm),
            machine.xregisters[s.rs2],
            WORD,
        )
    },
};
