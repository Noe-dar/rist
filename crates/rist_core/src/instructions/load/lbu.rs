use rist_macros::{bake_mask, bake_opcode};

use crate::debug;
use crate::memory::BYTE;

use crate::instructions::{types::itype::IType, Instruction};

pub const LBU: Instruction = Instruction {
    opcode: bake_opcode!(lbu),
    mask: bake_mask!(lbu),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: lbu reg(i.rd), reg(i.rs1), imm(i.imm));

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), BYTE);
    },
};
