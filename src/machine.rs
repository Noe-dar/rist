use crate::{bus::Bus, memory::WORD, registers::XRegisters};

pub const XLEN: usize = 64;

pub const LUI: usize = 0x37;
pub const IMM: usize = 0x13;
pub const OP: usize = 0x33;
pub const IMM_32: usize = 0x1B;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Machine {
    pub xregisters: XRegisters,
    pc: u64,
    bus: Bus,
}

impl Machine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn flash(&mut self, rom: &'static [u8]) {
        for (index, byte) in rom.iter().enumerate() {
            self.bus.rom[index] = *byte;
        }
    }

    pub fn fetch(&self) -> u32 {
        self.bus.read(self.pc, WORD) as u32
    }

    pub fn increment_pc(&mut self) {
        self.pc += 4;
    }

    pub fn execute(&mut self, instruction: u32) {
        let opcode = instruction & 0x7F;
        let rd = ((instruction & 0xF80) >> 7) as usize;
        let rs1 = ((instruction & 0xF8000) >> 15) as usize;
        let rs2 = ((instruction & 0x1f00000) >> 20) as usize;
        let funct3 = (instruction & 0x7000) >> 12;
        let funct7 = (instruction & 0xfe000000) >> 25;

        match opcode as usize {
            LUI => {
                let imm = instruction & 0xFFFFF000;
                println!("{}: LUI {}, {}", self.pc, rd, imm);
                self.xregisters[rd] = imm as u64;
            }
            OP => match (funct7, funct3) {
                (0, 0) => {
                    println!("{}: ADD {}, {}, {}", self.pc, rd, rs1, rs2);
                    self.xregisters[rd] = self.xregisters[rs1].wrapping_add(self.xregisters[rs2]);
                }
                _ => unimplemented!(),
            },
            IMM_32 => {
                let instruction = instruction as i32 as i64;
                
                match funct3 {
                    0x0 => {
                        let imm = instruction >> 20;
                        println!("{}: ADDI {}, {}, {}", self.pc, rd, rs1, imm);
                        self.xregisters[rd] = self.xregisters[rs1].wrapping_add(imm as u64);
                    }
                    _ => unimplemented!(),
                }
            }
            IMM => match funct3 {
                0x0 => {
                    let imm = (instruction & 0xFFF00000) >> 20;
                    println!("{}: ADDI {}, {}, {}", self.pc, rd, rs1, imm);
                    self.xregisters[rd] =
                        (self.xregisters[rs1].wrapping_add(imm as u64)) as i32 as i64 as u64
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}
