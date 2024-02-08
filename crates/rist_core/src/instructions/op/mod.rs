pub mod add;
pub mod sub;
mod addw;

pub use add::ADD;
pub use sub::SUB;

use self::addw::ADDW;

use super::Instruction;

pub const OP_INSTRUCTIONS: &[Instruction] = &[ADD, SUB, ADDW];