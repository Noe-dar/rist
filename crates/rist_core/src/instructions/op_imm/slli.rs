use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::itype::IType, Instruction}};

pub const SLLI: Instruction = Instruction {
    opcode: bake_opcode!(slli),
    mask: bake_mask!(slli),
    execute: |machine, word| {
        let i = IType::parse(word);
        let shamt: u64 = i.imm & 0xF;
        debug!(machine: andi reg(i.rd), reg(i.rs1), imm(shamt));

        machine.xregisters[i.rd] = (machine.xregisters[i.rs1] << shamt) as i32 as i64 as u64
    },
};
