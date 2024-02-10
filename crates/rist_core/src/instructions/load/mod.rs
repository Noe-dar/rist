pub mod lb;
pub mod lbu;
pub mod lh;
pub mod lhu;
pub mod lw;
pub mod ld;
pub mod lwu;

pub use lb::LB;
pub use lbu::LBU;
pub use lh::LH;
pub use lhu::LHU;
pub use lw::LW;

use self::ld::LD;

use super::Instruction;

pub const LOAD_INSTRUCTIONS: &[Instruction] = &[LB, LBU, LH, LHU, LW, LD];