use rist_macros::{bake_mask, bake_opcode};

use crate::debug;
use crate::memory::HALFWORD;

use crate::instructions::{types::itype::IType, Instruction};

pub const LHU: Instruction = Instruction {
    opcode: bake_opcode!(lhu),
    mask: bake_mask!(lhu),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: lhu reg(i.rd), reg(i.rs1), imm(i.imm));

        machine.xregisters[i.rd] =
            machine.bus.read(machine.xregisters[i.rs1].wrapping_add(i.imm), HALFWORD);
    },
};
