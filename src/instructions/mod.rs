use crate::machine::Machine;

pub mod lui;
pub mod types;

pub mod load;
pub mod op_imm;
pub mod op_imm_32;
pub mod op;

pub use load::*;
pub use op_imm::*;
pub use op_imm_32::*;
pub use op::*;

pub type InstructionExecute = fn(&mut Machine, u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction {
    pub opcode: u32,
    pub execute: InstructionExecute,
}

pub fn sext(value: u32) -> i64 {
    value as i32 as i64
}
