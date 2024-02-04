use std::collections::HashMap;

use phf::phf_map;

use crate::{exception::Exception, instructions::Instruction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoder {
    isa: &'static [Instruction],
    cache: HashMap<u32, Instruction>,
}

impl Decoder {
    const OPCODE_TO_MASK: phf::Map<u32, u32> = phf_map! {
        0x13_u32 => 0x707F,
        0x3B_u32 => 0x707F,
        0x3_u32 => 0x707F,
        
        0x33_u32 => 0xFE00707F,
        0x37_u32 => 0x7F,
        0x17_u32 => 0x7F,
    };

    pub fn new(isa: &'static [Instruction]) -> Self {
        Self { isa, cache: HashMap::with_capacity(isa.len()) }
    }

    pub fn decode(&mut self, instruction: u32) -> Result<Instruction, Exception> {
        let mask = Decoder::OPCODE_TO_MASK
            .get(&(instruction & 0x7f))
            .ok_or(Exception::IllegalInstruction)?;

        self.get_instruction(instruction & mask)
    }

    fn get_instruction(&mut self, opcode: u32) -> Result<Instruction, Exception> {
        if let Some(instruction) = self.cache.get(&opcode) {
            Ok(*instruction)
        } else {
            for instruction in self.isa {
                if opcode == instruction.opcode {
                    self.cache.insert(opcode, *instruction);
                    return Ok(*instruction);
                }
            }

            Err(Exception::IllegalInstruction)
        }
    }
}
