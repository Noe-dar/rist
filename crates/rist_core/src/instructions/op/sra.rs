use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::rtype::RType, Instruction}};

pub const SRA: Instruction = Instruction {
    opcode: bake_opcode!(sra),
    mask: bake_mask!(sra),
    execute: |machine, word| {
        let r = RType::parse(word);
        let shamt = machine.xregisters[r.rs2] & 0x3F;
        debug!(machine: sll reg(r.rd), reg(r.rs1), imm(shamt));

        machine.xregisters[r.rd] = ((machine.xregisters[r.rs1] as i64) << shamt) as u64;
    },
};
