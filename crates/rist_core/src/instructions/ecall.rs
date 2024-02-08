use rist_macros::{bake_mask, bake_opcode};

use super::Instruction;

pub const ECALL: Instruction = Instruction {
    opcode: bake_opcode!(ecall),
    mask: bake_mask!(ecall),
    execute: |machine, _| {
        println!("{}", machine.xregisters[5]);
    },
};
