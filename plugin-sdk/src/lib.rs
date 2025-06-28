use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub description: String,
    pub version: Version,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
