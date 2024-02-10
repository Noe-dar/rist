use rist_macros::{bake_mask, bake_opcode};

use super::Instruction;

pub const FENCE: Instruction = Instruction {
    opcode: bake_opcode!(fence),
    mask: bake_mask!(fence),
    execute: |_, _| {},
};