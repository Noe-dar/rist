use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Instruction {
    pub(crate) encoding: String,
    pub(crate) extension: Vec<String>,
    pub(crate) mask: String,
    #[serde(rename = "match")]
    pub(crate) opcode_match: String,
    pub(crate) variable_fields: Vec<String>
}

pub type InstructionTable = HashMap<String, Instruction>;