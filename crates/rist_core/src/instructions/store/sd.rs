use rist_macros::{bake_mask, bake_opcode};

use crate::{
    debug, instructions::{types::stype::SType, Instruction}, memory::DOUBLEWORD
};

pub const SD: Instruction = Instruction {
    opcode: bake_opcode!(sd),
    mask: bake_mask!(sd),
    execute: |machine, word| {
        let s = SType::parse(word);
        debug!(machine: sd reg(s.rs1), reg(s.rs2), imm(s.imm));

        machine.bus.write(
            machine.xregisters[s.rs1].wrapping_add(s.imm),
            machine.xregisters[s.rs2],
            DOUBLEWORD,
        )
    },
};
