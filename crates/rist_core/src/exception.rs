use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAMOAddressMisaligned,
    StoreAMOAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode = 11,
    InstructionPageFault,
    LoadPageFault,
    StoreAMOPageFault = 15,
}

impl Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exception::InstructionAddressMisaligned => write!(f, "INSTRUCTION_ADDRESS_MISALIGNED"),
            Exception::InstructionAccessFault => write!(f, "INSTRUCTION_ACCESS_FAULT"),
            Exception::IllegalInstruction => write!(f, "ILLEGAL_INSTRUCTION"),
            Exception::Breakpoint => write!(f, "BREAKPOINT"),
            Exception::LoadAddressMisaligned => write!(f, "LOAD_ADDRESS_MISALIGNED"),
            Exception::LoadAccessFault => write!(f, "LOAD_ACCESS_FAULT"),
            Exception::StoreAMOAddressMisaligned => write!(f, "STORE_AMO_ADDRESS_MISALIGNED"),
            Exception::StoreAMOAccessFault => write!(f, "STORE_AMO_ACCESS_FAULT"),
            Exception::EnvironmentCallFromUMode => write!(f, "ENVIRONMENT_CALL_FROM_UMODE"),
            Exception::EnvironmentCallFromSMode => write!(f, "ENVIRONMENT_CALL_FROM_SMODE"),
            Exception::EnvironmentCallFromMMode => write!(f, "ENVIRONMENT_CALL_FROM_MMODE"),
            Exception::InstructionPageFault => write!(f, "INSTRUCTION_PAGE_FAULT"),
            Exception::LoadPageFault => write!(f, "LOAD_PAGE_FAULT"),
            Exception::StoreAMOPageFault => write!(f, "STORE_AMO_PAGE_FAULT"),
        }
    }
}

impl Error for Exception {}
