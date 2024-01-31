use std::ops::Index;

pub const BYTE: u8 = 8;
pub const HALFWORD: u8 = 16;
pub const WORD: u8 = 32;
pub const DOUBLEWORD: u8 = 64;

pub trait MemoryExt: Index<usize, Output = u8> {
    fn read(&self, address: u64, size: u8) -> u64 {
        match size {
            BYTE => self.read_byte(address),
            HALFWORD => self.read_halfword(address),
            WORD => self.read_word(address),
            DOUBLEWORD => self.read_doubleword(address),
            _ => unimplemented!()
        }
    }

    fn read_byte(&self, address: u64) -> u64 {
        let address = address as usize;

        self[address] as u64
    }

    fn read_halfword(&self, address: u64) -> u64 {
        let address = address as usize;

        self[address] as u64 | (self[address + 1] as u64) << 8
    }

    fn read_word(&self, address: u64) -> u64 {
        let address = address as usize;

        self[address] as u64
            | (self[address + 1] as u64) << 8
            | (self[address + 2] as u64) << 16
            | (self[address + 3] as u64) << 24
    }

    fn read_doubleword(&self, address: u64) -> u64 {
        let address = address as usize;

        self[address] as u64
            | (self[address + 1] as u64) << 8
            | (self[address + 2] as u64) << 16
            | (self[address + 3] as u64) << 24
            | (self[address + 4] as u64) << 32
            | (self[address + 5] as u64) << 40
            | (self[address + 6] as u64) << 48
            | (self[address + 7] as u64) << 56
    }
}

impl<T: Index<usize, Output = u8>> MemoryExt for T {}
