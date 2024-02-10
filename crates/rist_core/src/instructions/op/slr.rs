use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::rtype::RType, Instruction}};

pub const SLR: Instruction = Instruction {
    opcode: bake_opcode!(srl),
    mask: bake_mask!(srl),
    execute: |machine, word| {
        let r = RType::parse(word);
        let shamt = machine.xregisters[r.rs2] & 0x3F;
        debug!(machine: sll reg(r.rd), reg(r.rs1), imm(shamt));

        machine.xregisters[r.rd] = machine.xregisters[r.rs1] >> shamt;
    },
};
