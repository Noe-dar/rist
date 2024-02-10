pub mod sd;
pub mod sb;
pub mod sh;
pub mod sw;

pub use sd::SD;
pub use sw::SW;
pub use sh::SH;
pub use sb::SB;

use super::Instruction;

pub const STORE_INSTRUCTIONS: &[Instruction] = &[SD, SW, SH, SB];