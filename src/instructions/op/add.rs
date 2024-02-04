use crate::instructions::{types::rtype::RType, Instruction};

pub const ADD: Instruction = Instruction {
    opcode: 0x33,
    execute: |machine, word| {
        let r = RType::parse(word);

        machine.xregisters[r.rd] = machine.xregisters[r.rs1].wrapping_add(machine.xregisters[r.rs2])
    },
};
