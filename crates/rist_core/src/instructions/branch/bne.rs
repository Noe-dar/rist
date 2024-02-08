use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::btype::BType, Instruction}};

pub const BNE: Instruction = Instruction {
    opcode: bake_opcode!(bne),
    mask: bake_mask!(bne),
    execute: |machine, word| {
        let b = BType::parse(word);
        debug!(machine: bne reg(b.rs1), reg(b.rs2), imm(b.imm));

        if machine.xregisters[b.rs1] != machine.xregisters[b.rs2] {
            machine.pc = machine.pc.wrapping_add(b.imm).wrapping_sub(4)
        }
    },
};