use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod elements;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub version: Version,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UINode {
    pub name: String,
    pub props: HashMap<String, String>,
    pub children: Vec<UINode>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Mount,
    Event { event: String, data: ActionData },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ActionData {
    pub text_inputs: HashMap<String, String>,
    pub radio_groups: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StateInput<T> {
    pub action: Action,
    pub old_state: Option<T>,
}
