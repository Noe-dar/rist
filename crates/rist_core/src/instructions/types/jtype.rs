use crate::instructions::sext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JType {
    pub rd: usize,
    pub imm: u64,
}

impl JType {
    pub fn parse(instruction: u32) -> Self {
        let imm = sext(
            (instruction & 0xff000) | (instruction & 0x100000) >> 9 | (instruction & 0x7fe00000) >> 20,
        ) as u64;

        let rd = ((instruction & 0xf80) >> 7) as usize;

        Self { rd, imm }
    }
}
