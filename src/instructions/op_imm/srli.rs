use crate::instructions::{types::itype::IType, Instruction, SRAI};

pub const SRLI: Instruction = Instruction {
    opcode: 0x13 | 0x5 << 12,
    execute: |machine, word| {
        let i = IType::parse(word);
        let imm = (i.imm & 0xfffffff0) >> 5;
        let shamt: u64 = i.imm & 0xF;

        if imm == 0x20 {
            (SRAI.execute)(machine, word);
            return;
        }

        machine.xregisters[i.rd] = (machine.xregisters[i.rs1] >> shamt) as i32 as i64 as u64
    },
};
