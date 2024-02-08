use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::btype::BType, Instruction}};

pub const BGE: Instruction = Instruction {
    opcode: bake_opcode!(bge),
    mask: bake_mask!(bge),
    execute: |machine, word| {
        let b = BType::parse(word);
        debug!(machine: bge reg(b.rs1), reg(b.rs2), imm(b.imm));
        
        if (machine.xregisters[b.rs1] as i64) > (machine.xregisters[b.rs2] as i64) {
            machine.pc = machine.pc.wrapping_add(b.imm).wrapping_sub(4)
        }
    },
};