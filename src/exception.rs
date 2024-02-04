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
