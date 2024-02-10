use std::{ops::{Index, IndexMut}, slice::Iter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct XRegisters {
    registers: [u64; 32],
    zero: u64,
}

impl XRegisters {
    pub fn iter(&self) -> Iter<'_, u64> {
        self.registers.iter()
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