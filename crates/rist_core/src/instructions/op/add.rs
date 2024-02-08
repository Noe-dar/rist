use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::rtype::RType, Instruction}};

pub const ADD: Instruction = Instruction {
    opcode: bake_opcode!(add),
    mask: bake_mask!(add),
    execute: |machine, word| {
        let r = RType::parse(word);
        debug!(machine: add reg(r.rd), reg(r.rs1), reg(r.rs2));

        machine.xregisters[r.rd] = machine.xregisters[r.rs1].wrapping_add(machine.xregisters[r.rs2])
    },
};
