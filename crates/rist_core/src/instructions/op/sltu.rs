use rist_macros::{bake_mask, bake_opcode};

use crate::{
    debug,
    instructions::{types::rtype::RType, Instruction},
};

pub const SLTU: Instruction = Instruction {
    opcode: bake_opcode!(sltu),
    mask: bake_mask!(sltu),
    execute: |machine, word| {
        let r = RType::parse(word);
        debug!(machine: sltu reg(r.rd), reg(r.rs1), reg(r.rs2));

        machine.xregisters[r.rd] =
            if machine.xregisters[r.rs1] < machine.xregisters[r.rs2] { 1 } else { 0 }
    },
};
