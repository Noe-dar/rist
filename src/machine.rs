use crate::{
    bus::Bus, decoder::Decoder, exception::Exception, instructions::Instruction, memory::WORD,
    registers::XRegisters,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    pub xregisters: XRegisters,
    pub pc: u64,
    pub(crate) bus: Bus,
    decoder: Decoder,
}

impl Machine {
    pub fn new(isa: &'static [Instruction]) -> Self {
        Self {
            pc: 0,
            xregisters: XRegisters::default(),
            bus: Bus::default(),
            decoder: Decoder::new(isa),
        }
    }

    pub fn flash(&mut self, rom: &'static [u8]) {
        for (index, byte) in rom.iter().enumerate() {
            self.bus.rom[index] = *byte;
        }
    }

    pub fn fetch(&mut self) -> u32 {
        let instruction = self.bus.read(self.pc, WORD) as u32;
        self.pc += 4;
        instruction
    }

    pub fn decode(&mut self, instruction: u32) -> Result<Instruction, Exception> {
        self.decoder.decode(instruction)
    }
}
