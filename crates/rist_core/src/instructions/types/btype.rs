#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BType {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u64,
}

impl BType {
    pub fn parse(instruction: u32) -> Self {
        let imm = (((instruction & 0x80000000) as i32 as i64 >> 19) as u64)
        | ((instruction as u64 & 0x80) << 4) // imm[11]
        | ((instruction as u64 >> 20) & 0x7e0) // imm[10:5]
        | ((instruction as u64 >> 7) & 0x1e);

        let rs1 = ((instruction & 0xf8000) >> 15) as usize;
        let rs2 = ((instruction & 0x1F00000) >> 20) as usize;

        Self { rs1, rs2, imm }
    }
}
