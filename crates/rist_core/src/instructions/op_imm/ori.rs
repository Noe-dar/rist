use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::itype::IType, Instruction}};

pub const ORI: Instruction = Instruction {
    opcode: bake_opcode!(ori),
    mask: bake_mask!(ori),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: ori reg(i.rd), reg(i.rs1), imm(i.imm));

        machine.xregisters[i.rd] = machine.xregisters[i.rs1] | i.imm
    },
};
