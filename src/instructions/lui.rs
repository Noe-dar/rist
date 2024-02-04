use super::{sext, types::itype::IType, Instruction};

pub const LUI: Instruction = Instruction {
    opcode: 0x37,
    execute: |machine, word| {
        let i = IType::parse(word);
        let imm = sext(word) & 0xFFFFF000;

        machine.xregisters[i.rd] = imm as u64
    },
};
