use rist_macros::{bake_mask, bake_opcode};

use crate::debug;
use crate::memory::DOUBLEWORD;

use crate::instructions::{types::itype::IType, Instruction};

pub const LD: Instruction = Instruction {
    opcode: bake_opcode!(ld),
    mask: bake_mask!(ld),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: lw reg(i.rd), reg(i.rs1), imm(i.imm));

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), DOUBLEWORD) as i32 as i64
                as u64;
    },
};
