use std::collections::HashMap;

use crate::{exception::Exception, instructions::Instruction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoder {
    isa: &'static [Instruction],
    cache: HashMap<u32, Instruction>,
}

impl Decoder {
    pub fn new(isa: &'static [Instruction]) -> Self {
        Self { isa, cache: HashMap::with_capacity(isa.len()) }
    }

    pub fn decode(&mut self, word: u32) -> Result<Instruction, Exception> {
        if let Some(instruction) = self.cache.get(&word) {
            Ok(*instruction)
        } else {
            for instruction in self.isa {
                if word & instruction.mask == instruction.opcode {
                    self.cache.insert(word, *instruction);
                    return Ok(*instruction);
                }
            }

            Err(Exception::IllegalInstruction)
        }
    }
}
