#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RType {
    pub rd: usize,
    pub funct3: u32,
    pub rs1: usize,
    pub rs2: usize,
}

impl RType {
    pub fn parse(instruction: u32) -> Self {
        let rd = ((instruction & 0xf80) >> 7) as usize;
        let funct3 = (instruction & 0x7000) >> 12;
        let rs1 = ((instruction & 0xf8000) >> 15) as usize;
        let rs2 = ((instruction & 0x1f00000) >> 20) as usize;

        Self { rd, funct3, rs1, rs2  }
    }
}
