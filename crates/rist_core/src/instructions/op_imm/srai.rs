use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::itype::IType, Instruction}};

pub const SRAI: Instruction = Instruction {
    opcode: bake_opcode!(srai),
    mask: bake_mask!(srai),
    execute: |machine, word| {
        let i = IType::parse(word);
        let shamt: u64 = i.imm & 0xF;
        debug!(machine: sltiu reg(i.rd), reg(i.rs1), imm(shamt));

        machine.xregisters[i.rd] = machine.xregisters[i.rs1] >> shamt
    },
};
