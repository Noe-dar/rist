use std::{ops::{Index, IndexMut}, slice::Iter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct XRegisters {
    registers: [u64; 32],
    zero: u64,
}

impl XRegisters {
    const ABI_NAMES: [&'static str; 32] = [
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
        "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
        "t5", "t6",
    ];

    pub fn iter(&self) -> Iter<'_, u64> {
        self.registers.iter()
    }

    pub fn dump(&self) {
        for register in (0..32).step_by(2) {
            print!(
                "{name}:\t{register:#x}\t(DECIMAL={register})\t",
                register = self.registers[register],
                name = XRegisters::ABI_NAMES[register]
            );
            print!(
                "{name}:\t{register:#x}\t(DECIMAL={register})",
                register = self.registers[register + 1],
                name = XRegisters::ABI_NAMES[register + 1]
            );
            println!()
        }
    }
}

impl Index<usize> for XRegisters {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &0,
            index => &self.registers[index],
        }
    }
}

impl IndexMut<usize> for XRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.zero,
            index => &mut self.registers[index],
        }
    }
}