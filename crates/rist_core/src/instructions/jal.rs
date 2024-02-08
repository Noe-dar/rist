use rist_macros::{bake_mask, bake_opcode};

use crate::debug;

use super::{types::jtype::JType, Instruction};

pub const JAL: Instruction = Instruction {
    opcode: bake_opcode!(jal),
    mask: bake_mask!(jal),
    execute: |machine, word| {
        let j = JType::parse(word);
        debug!(machine: jal reg(j.rd), imm(j.imm));

        machine.xregisters[j.rd] = machine.pc.wrapping_add(4);
        machine.pc = machine.pc.wrapping_add(j.imm)
    },
};