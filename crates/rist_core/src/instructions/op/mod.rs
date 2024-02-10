pub mod add;
pub mod sub;
pub mod addw;
pub mod sll;
pub mod slr;
pub mod sra;
pub mod sltu;
pub mod slt;
pub mod xor;
pub mod or;
pub mod and;

pub use add::ADD;
pub use sub::SUB;
pub use addw::ADDW;
pub use sll::SLL;
pub use slr::SLR;
pub use sra::SRA;
pub use sltu::SLTU;
pub use slt::SLT;
pub use xor::XOR;
pub use or::OR;
pub use and::AND;

use super::Instruction;

pub const OP_INSTRUCTIONS: &[Instruction] = &[ADD, SUB, ADDW, SLL, SLR, SRA, SLTU, SLT, XOR, OR, AND    ];