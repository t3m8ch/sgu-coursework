use extism_pdk::*;
use plugin_sdk::{
    elements::{fragment, row, text, FontWeight, TextSize},
    fragment, row, text, PluginMetadata, UINode, Version,
};

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
    Ok(Json(fragment!(&[
        row!(&[text!(
            "Теория графов",
            size = TextSize::Large,
            weight = FontWeight::Bold
        )]),
        row!(&[text!(
            "Что такое граф?",
            size = TextSize::Large,
            weight = FontWeight::Medium
        )]),
    ])))
}
