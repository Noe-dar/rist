use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::rtype::RType, Instruction}};

pub const SUB: Instruction = Instruction {
    opcode: bake_opcode!(sub),
    mask: bake_mask!(sub),
    execute: |machine, word| {
        let r = RType::parse(word);
        debug!(machine: sub reg(r.rd), reg(r.rs1), reg(r.rs2));

        machine.xregisters[r.rd] =
            (machine.xregisters[r.rs1] as i64).wrapping_sub(machine.xregisters[r.rs2] as i64) as i64 as u64;

        println!("{}", machine.xregisters[r.rd] as i64)
    },
};
