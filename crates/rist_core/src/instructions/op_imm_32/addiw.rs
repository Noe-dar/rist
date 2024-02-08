use rist_macros::{bake_mask, bake_opcode};

use crate::{
    debug,
    instructions::{types::itype::IType, Instruction},
};

pub const ADDIW: Instruction = Instruction {
    opcode: bake_opcode!(addiw),
    mask: bake_mask!(addiw),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: addiw reg(i.rd), reg(i.rs1), simm(i.imm as i32 as i64));

        machine.xregisters[i.rd] =
            (machine.xregisters[i.rs1] as i64).wrapping_add(i.imm as i32 as i64) as u64;
    },
};
