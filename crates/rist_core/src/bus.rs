use crate::memory::{ReadMemoryExt, WriteMemoryExt};

const ROM_SIZE: usize = 4 * 1024;
const DRAM_SIZE: usize = 16 * 1024;

const ROM_BASE: usize = 0x0;
const ROM_END: usize = ROM_SIZE;

const DRAM_BASE: usize = 0x8000;
const DRAM_END: usize = DRAM_BASE + DRAM_BASE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bus {
    pub rom: [u8; ROM_SIZE],
    dram: [u8; DRAM_SIZE],
}

impl Default for Bus {
    fn default() -> Self {
        Self { rom: [0; ROM_SIZE], dram: [0; DRAM_SIZE] }
    }
}

impl Bus {
    pub fn read(&self, address: u64, size: u8) -> u64 {
        match address as usize {
            ROM_BASE..=ROM_END => self.rom.read(address, size),
            DRAM_BASE..=DRAM_END => self.dram.read(address - DRAM_BASE as u64, size),
            _ => unimplemented!(),
        }
    }

    pub fn write(&mut self, address: u64, value: u64, size: u8) {
        match address as usize {
            ROM_BASE..=ROM_END => unimplemented!(),
            DRAM_BASE..=DRAM_END => self.dram.write(address - DRAM_BASE as u64, value, size),
            _ => unimplemented!(),
        }
    }
}
