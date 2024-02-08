use crate::{
    bus::Bus, debug::Debugger, decoder::Decoder, exception::Exception, instructions::Instruction, memory::WORD, registers::XRegisters
};

#[derive(Clone)]
pub struct Machine {
    pub(crate) xregisters: XRegisters,
    pub(crate) pc: u64,
    pub(crate) bus: Bus,
    decoder: Decoder,
    pub(crate) debugger: Option<&'static dyn Debugger>,
}

impl Machine {
    pub fn new(isa: &'static [Instruction]) -> Self {
        Self {
            pc: 0,
            xregisters: XRegisters::default(),
            bus: Bus::default(),
            decoder: Decoder::new(isa),
            debugger: None
        }
    }

    pub fn attach_debugger(&mut self, debugger: &'static dyn Debugger) {
        self.debugger = Some(debugger)
    }

    pub fn flash(&mut self, rom: &'static [u8]) {
        for (index, byte) in rom.iter().enumerate() {
            self.bus.rom[index] = *byte;
        }
    }

    pub fn fetch(&mut self) -> u32 {
        self.bus.read(self.pc, WORD) as u32
    }

    pub fn decode(&mut self, instruction: u32) -> Result<Instruction, Exception> {
        self.decoder.decode(instruction)
    }

    pub fn dump(&self) {
        if let Some(debugger) = self.debugger {
            debugger.dump(self.pc, &self.xregisters).unwrap();
        }
    }
}
