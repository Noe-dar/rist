use rist_macros::{bake_mask, bake_opcode};

use crate::{
    debug,
    instructions::{types::btype::BType, Instruction},
};

pub const BLTU: Instruction = Instruction {
    opcode: bake_opcode!(bltu),
    mask: bake_mask!(bltu),
    execute: |machine, word| {
        let b = BType::parse(word);
        debug!(machine: bltu reg(b.rs1), reg(b.rs2), simm(b.imm as i64));

        if machine.xregisters[b.rs1] < machine.xregisters[b.rs2] {
            machine.pc = machine.pc.wrapping_add(b.imm).wrapping_sub(4)
        }
    },
};
