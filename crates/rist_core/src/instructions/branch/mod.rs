pub mod beq;
pub mod bge;
pub mod bgeu;
pub mod blt;
pub mod bltu;
pub mod bne;

pub use beq::BEQ;
pub use bge::BGE;
pub use bgeu::BGEU;
pub use blt::BLT;
pub use bltu::BLTU;
pub use bne::BNE;

use super::Instruction;

pub const BRANCH_INSTRUCTIONS: &[Instruction] = &[BEQ, BNE, BLT, BLTU, BGE, BGEU];
