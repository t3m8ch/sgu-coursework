use std::collections::HashMap;

use extism_pdk::*;
use plugin_sdk::{PluginMetadata, UINode, Version};

#[plugin_fn]
pub fn metadata() -> FnResult<Json<PluginMetadata>> {
    Ok(Json(PluginMetadata {
        name: "simple-plugin".to_string(),
        display_name: "Simple Plugin".to_string(),
        description: "A simple plugin".to_string(),
        version: Version {
            major: 0,
            minor: 1,
            patch: 0,
        },
    }))
}

#[plugin_fn]
pub fn ui() -> FnResult<Json<UINode>> {
    Ok(Json(UINode {
        name: "fragment".to_string(),
        props: HashMap::new(),
        children: vec![
            UINode {
                name: "row".to_string(),
                props: HashMap::new(),
                children: vec![UINode {
                    name: "text".to_string(),
                    props: HashMap::from([
                        ("size".to_string(), "large".to_string()),
                        ("weight".to_string(), "bold".to_string()),
                        ("text".to_string(), "Теория графов".to_string()),
                    ]),
                    children: vec![],
                }],
            },
            UINode {
                name: "row".to_string(),
                props: HashMap::new(),
                children: vec![UINode {
                    name: "text".to_string(),
                    props: HashMap::from([
                        ("size".to_string(), "medium".to_string()),
                        ("weight".to_string(), "medium".to_string()),
                        ("text".to_string(), "Что такое граф?".to_string()),
                    ]),
                    children: vec![],
                }],
            },
        ],
    }))
}
