use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::itype::IType, Instruction}};

pub const SLLIW: Instruction = Instruction {
    opcode: bake_opcode!(slliw),
    mask: bake_mask!(slliw),
    execute: |machine, word| {
        let i = IType::parse(word);
        let shamt = (i.imm & 0x1F) as u32;
        debug!(machine: slli reg(i.rd), reg(i.rs1), imm(shamt as u64));

        machine.xregisters[i.rd] = (machine.xregisters[i.rs1] << shamt) as i32 as i64 as u64
    },
};
