use crate::instructions::sext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IType {
    pub rd: usize,
    pub funct3: u32,
    pub rs1: usize,
    pub imm: u64,
}

impl IType {
    pub fn parse(instruction: u32) -> Self {
        let rd = ((instruction & 0xf80) >> 7) as usize;
        let funct3 = (instruction & 0x7000) >> 12;
        let rs1 = ((instruction & 0xf8000) >> 15) as usize;
        let imm = (sext(instruction) >> 20) as u64;

        Self { rd, funct3, rs1, imm }
    }
}
