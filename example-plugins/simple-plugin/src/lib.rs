use extism_pdk::*;
use plugin_sdk::{PluginMetadata, Version};

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
