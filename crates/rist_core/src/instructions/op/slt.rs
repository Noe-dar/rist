use rist_macros::{bake_mask, bake_opcode};

use crate::{
    debug,
    instructions::{types::rtype::RType, Instruction},
};

pub const SLT: Instruction = Instruction {
    opcode: bake_opcode!(slt),
    mask: bake_mask!(slt),
    execute: |machine, word| {
        let r = RType::parse(word);
        debug!(machine: slt reg(r.rd), reg(r.rs1), reg(r.rs2));

        machine.xregisters[r.rd] =
            if (machine.xregisters[r.rs1] as i64) < (machine.xregisters[r.rs2] as i64) { 1 } else { 0 }
    },
};
