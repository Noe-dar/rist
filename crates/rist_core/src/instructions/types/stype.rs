use crate::instructions::sext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SType {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u64
}

impl SType {
    pub fn parse(instruction: u32) -> Self {
        let imm = sext(
            (instruction >> 20) & 0xfe0 | (instruction >> 7) & 0x1f,
        ) as u64;

        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;


        Self { rs1, rs2, imm }
    }
}
