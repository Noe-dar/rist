use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::itype::IType, Instruction}};

pub const XORI: Instruction = Instruction {
    opcode: bake_opcode!(xori),
    mask: bake_mask!(xori),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: xori reg(i.rd), reg(i.rs1), imm(i.imm));

        machine.xregisters[i.rd] = machine.xregisters[i.rs1] ^ i.imm;
    },
};
