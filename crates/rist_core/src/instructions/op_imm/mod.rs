pub mod addi;
pub mod andi;
pub mod ori;
pub mod slli;
pub mod slti;
pub mod sltiu;
pub mod srai;
pub mod srli;
pub mod xori;

pub use addi::ADDI;
pub use andi::ANDI;
pub use ori::ORI;
pub use slli::SLLI;
pub use slti::SLTI;
pub use sltiu::SLTIU;
pub use srai::SRAI;
pub use srli::SRLI;
pub use xori::XORI;

use super::Instruction;

pub const IMM_INSTRUCTIONS: &[Instruction] = &[ADDI, ANDI, ORI, SLLI, SLTI, SLTIU, SRAI, SRLI, XORI];
