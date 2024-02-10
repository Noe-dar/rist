use std::ops::{Index, IndexMut};

pub const BYTE: u8 = 8;
pub const HALFWORD: u8 = 16;
pub const WORD: u8 = 32;
pub const DOUBLEWORD: u8 = 64;

pub(crate) trait WriteMemoryExt: IndexMut<usize, Output = u8> {
    fn write(&mut self, address: u64, value: u64, size: u8) {
        match size {
            BYTE => self.write_byte(address, value),
            HALFWORD => self.write_halfword(address, value),
            WORD => self.write_halfword(address, value),
            DOUBLEWORD => self.write_halfword(address, value),
            _ => unimplemented!(),
        }
    }

    fn write_byte(&mut self, address: u64, value: u64) {
        let address = address as usize;

        self[address] = (value & 0xff) as u8;
    }

    fn write_halfword(&mut self, address: u64, value: u64) {
        let address = address as usize;

        self[address] = (value & 0xff) as u8;
        self[address + 1] = ((value >> 8) & 0xff) as u8
    }

    fn write_word(&mut self, address: u64, value: u64) {
        let address = address as usize;

        self[address] = (value & 0xff) as u8;
        self[address + 1] = ((value >> 8) & 0xff) as u8;
        self[address + 2] = ((value >> 16) & 0xff) as u8;
        self[address + 3] = ((value >> 24) & 0xff) as u8
    }

    fn write_doubleword(&mut self, address: u64, value: u64) {
        let address = address as usize;

        self[address] = (value & 0xff) as u8;
        self[address + 1] = ((value >> 8) & 0xff) as u8;
        self[address + 2] = ((value >> 16) & 0xff) as u8;
        self[address + 3] = ((value >> 24) & 0xff) as u8;
        self[address + 4] = ((value >> 32) & 0xff) as u8;
        self[address + 5] = ((value >> 40) & 0xff) as u8;
        self[address + 6] = ((value >> 48) & 0xff) as u8;
        self[address + 6] = ((value >> 56) & 0xff) as u8
    }
}

pub trait ReadMemoryExt: Index<usize, Output = u8> {
    fn read(&self, address: u64, size: u8) -> u64 {
        match size {
            BYTE => self.read_byte(address),
            HALFWORD => self.read_halfword(address),
            WORD => self.read_word(address),
            DOUBLEWORD => self.read_doubleword(address),
            _ => unimplemented!(),
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

impl<T: Index<usize, Output = u8>> ReadMemoryExt for T {}
impl<T: IndexMut<usize, Output = u8>> WriteMemoryExt for T {}
