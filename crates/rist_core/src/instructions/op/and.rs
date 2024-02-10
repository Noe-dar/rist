use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::rtype::RType, Instruction}};

pub const AND: Instruction = Instruction {
    opcode: bake_opcode!(and),
    mask: bake_mask!(and),
    execute: |machine, word| {
        let r = RType::parse(word);
        debug!(machine: and reg(r.rd), reg(r.rs1), reg(r.rs2));

        machine.xregisters[r.rd] = machine.xregisters[r.rs1] & machine.xregisters[r.rs2];
    },
};
