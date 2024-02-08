use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::itype::IType, Instruction}};

pub const SRLI: Instruction = Instruction {
    opcode: bake_opcode!(srli),
    mask: bake_mask!(srli),
    execute: |machine, word| {
        let i = IType::parse(word);
        let shamt = i.imm & 0xF;

        debug!(machine: srli reg(i.rd), reg(i.rs1), imm(shamt));

        machine.xregisters[i.rd] = (machine.xregisters[i.rs1] >> shamt) as i32 as i64 as u64
    },
};
