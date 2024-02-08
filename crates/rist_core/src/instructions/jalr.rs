use rist_macros::{bake_mask, bake_opcode};

use crate::debug;

use super::{types::itype::IType, Instruction};

pub const JALR: Instruction = Instruction {
    opcode: bake_opcode!(jalr),
    mask: bake_mask!(jalr),
    execute: |machine, word| {
        let i = IType::parse(word);
        debug!(machine: jalr reg(i.rd), reg(i.rs1), imm(i.imm));

        let tmp = machine.pc;
        machine.pc = machine.xregisters[i.rs1].wrapping_add(i.imm);
        machine.xregisters[i.rd] = tmp;
    },
};