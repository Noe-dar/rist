use rist_macros::{bake_mask, bake_opcode};

use crate::debug;
use crate::memory::WORD;

use crate::instructions::{types::itype::IType, Instruction};

pub const LWU: Instruction = Instruction {
    opcode: bake_opcode!(lwu),
    mask: bake_mask!(lwu),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: lw reg(i.rd), reg(i.rs1), imm(i.imm));

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), WORD);
    },
};
