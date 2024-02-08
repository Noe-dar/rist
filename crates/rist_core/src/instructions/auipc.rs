use rist_macros::{bake_mask, bake_opcode};

use crate::debug;

use super::Instruction;

pub const AUIPC: Instruction = Instruction {
    opcode: bake_opcode!(auipc),
    mask: bake_mask!(auipc),
    execute: |machine, word| {
        let rd = ((word >> 7) & 0x1f) as usize;
        let imm = word as u64 & 0xfffff000;

        debug!(machine: auipc reg(rd), imm(imm >> 12));

        machine.xregisters[rd] = machine.pc.wrapping_add(imm).wrapping_sub(4);
    },
};