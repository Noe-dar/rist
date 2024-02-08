use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::itype::IType, Instruction}};

pub const SLTIU: Instruction = Instruction {
    opcode: bake_opcode!(sltiu),
    mask: bake_mask!(sltiu),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: sltiu reg(i.rd), reg(i.rs1), imm(i.imm));
        
        machine.xregisters[i.rd] = if machine.xregisters[i.rs1] < i.imm { 1 } else { 0 }
    },
};
