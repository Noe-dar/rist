use rist_macros::{bake_mask, bake_opcode};

use crate::{
    debug,
    instructions::{types::itype::IType, Instruction},
};

pub const ANDI: Instruction = Instruction {
    opcode: bake_opcode!(andi),
    mask: bake_mask!(andi),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: andi reg(i.rd), reg(i.rs1), imm(i.imm));

        machine.xregisters[i.rd] = machine.xregisters[i.rs1] & i.imm
    },
};
