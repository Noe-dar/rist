use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::itype::IType, Instruction}};

pub const SLTI: Instruction = Instruction {
    opcode: bake_opcode!(slti),
    mask: bake_mask!(slti),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: slti reg(i.rd), reg(i.rs1), imm(i.imm));

        machine.xregisters[i.rd] =
            if (machine.xregisters[i.rs1] as i64) < (i.imm as i64) { 1 } else { 0 }
    },
};
