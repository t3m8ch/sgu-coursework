use std::{
    fs,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use extism::convert::Json;
use plugin_sdk::{PluginMetadata, UINode};

#[derive(Debug, Clone)]
pub struct Plugin {
    pub metadata: plugin_sdk::PluginMetadata,
    extism_plugin: Arc<Mutex<extism::Plugin>>,
}

impl Plugin {
    pub fn load_from_dir(dir: &str) -> anyhow::Result<Vec<anyhow::Result<Plugin>>> {
        log::debug!("Scanning plugin directory: {}", dir);

        Ok(fs::read_dir(dir)?
            .filter_map(|file| file.ok())
            .filter_map(|file| file.file_type().ok().zip(Some(file)))
            .filter(|(file_type, _)| file_type.is_file())
            .map(|(_, file)| {
                let file_path = file.path();
                let file_err_ctx = || {
                    format!(
                        "Failed to create plugin from file: {:?}",
                        file_path.to_str()
                    )
                };
                log::debug!("Processing plugin file: {:?}", file_path);

                let wasm = extism::Wasm::file(&file_path);
                let manifest = extism::Manifest::new([wasm]);
                let mut extism_plugin =
                    extism::Plugin::new(&manifest, [], true).with_context(file_err_ctx)?;

                let Json(metadata): Json<PluginMetadata> = extism_plugin
                    .call("metadata", ())
                    .with_context(file_err_ctx)?;

                let extism_plugin = Arc::new(Mutex::new(extism_plugin));
                Ok(Plugin {
                    metadata,
                    extism_plugin,
                })
            })
            .collect())
    }

    pub fn ui(&mut self) -> anyhow::Result<UINode> {
        let Json(ui): Json<UINode> = self
            .extism_plugin
            .lock()
            .unwrap()
            .call("ui", ())
            .with_context(|| {
                format!(
                    "Failed to call 'ui' function for plugin: {}",
                    self.metadata.name
                )
            })?;

        Ok(ui)
    }
}
