use rist_macros::{bake_mask, bake_opcode};

use crate::debug;

use super::{sext, types::itype::IType, Instruction};

pub const LUI: Instruction = Instruction {
    opcode: bake_opcode!(lui),
    mask: bake_mask!(lui),
    execute: |machine, word| {
        let i = IType::parse(word);
        let imm = sext(word) & 0xFFFFF000;
        debug!(machine: lui reg(i.rd), simm(imm >> 12));

        machine.xregisters[i.rd] = imm as u64
    },
};
