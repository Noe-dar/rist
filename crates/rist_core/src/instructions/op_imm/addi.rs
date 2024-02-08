use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::itype::IType, Instruction}};

pub const ADDI: Instruction = Instruction {
    opcode: bake_opcode!(addi),
    mask: bake_mask!(addi),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: addi reg(i.rd), reg(i.rs1), simm(i.imm as i64));

        machine.xregisters[i.rd] = machine.xregisters[i.rs1].wrapping_add(i.imm);
    },
};
