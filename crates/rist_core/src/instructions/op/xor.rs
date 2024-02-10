use rist_macros::{bake_mask, bake_opcode};

use crate::{debug, instructions::{types::rtype::RType, Instruction}};

pub const XOR: Instruction = Instruction {
    opcode: bake_opcode!(xor),
    mask: bake_mask!(xor),
    execute: |machine, word| {
        let r = RType::parse(word);
        debug!(machine: xor reg(r.rd), reg(r.rs1), reg(r.rs2));

        machine.xregisters[r.rd] = machine.xregisters[r.rs1] ^ machine.xregisters[r.rs2];
    },
};
