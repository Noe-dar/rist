pub mod addiw;
pub use addiw::ADDIW;

use super::Instruction;

pub const IMM_32_INSTRUCTIONS: &[Instruction] = &[ADDIW];