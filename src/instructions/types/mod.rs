use phf::phf_map;

pub mod itype;
pub mod rtype;

pub const OPCODE_TO_MASK: phf::Map<u32, u32> = phf_map! {
    0x13_u32 => 0x707F
};
