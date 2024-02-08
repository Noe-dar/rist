use crate::machine::Machine;

pub mod lui;
pub mod types;

pub mod branch;
pub mod jal;
pub mod load;
pub mod op;
pub mod op_imm;
pub mod op_imm_32;
pub mod store;
pub mod jalr;
pub mod auipc;
pub mod ecall;

pub use load::*;
pub use op::*;
pub use op_imm::*;
pub use op_imm_32::*;

pub type InstructionExecute = fn(&mut Machine, u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction {
    pub opcode: u32,
    pub mask: u32,
    pub execute: InstructionExecute,
}

impl Instruction {
    pub const EMPTY: Instruction = Instruction {
        opcode: 0x0,
        mask: 0x0,
        execute: |_, _| {},
    };
}

pub fn sext(value: u32) -> i64 {
    value as i32 as i64
}
