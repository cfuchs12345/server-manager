use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct ArgDef {
    pub arg_type: String,
    pub value: String,
}

impl PartialEq for ArgDef {
    fn eq(&self, other: &Self) -> bool {
        self.arg_type == other.arg_type
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Script {
    pub script_type: String,
    pub script: String,
}
