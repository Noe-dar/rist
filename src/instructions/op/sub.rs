use crate::instructions::{types::rtype::RType, Instruction};

pub const SUB: Instruction = Instruction {
    opcode: 0x33 | 0x20 << 25,
    execute: |machine, word| {
        let r = RType::parse(word);

        machine.xregisters[r.rd] =
            (machine.xregisters[r.rs1] as i64).wrapping_sub(machine.xregisters[r.rs2] as i64) as i64 as u64;

        println!("{}", machine.xregisters[r.rd] as i64)
    },
};
