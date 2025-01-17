use std::collections::BTreeMap;

use anyhow::Result;
use hhaven_plugin::{Outgoing, Plugin, PluginConfig, PLUGIN_ENTRYPOINT_NAME};

use crate::Config;

pub struct PluginLoader {
    plugins: BTreeMap<String, Box<Plugin>>,
    _librarys: BTreeMap<String, dlopen::symbor::Library>,
}

impl PluginLoader {
    pub fn new(configs: Vec<Config>) -> Result<Self> {
        let mut plugins = BTreeMap::new();
        let mut _librarys = BTreeMap::new();

        for config in configs {
            let library = dlopen::symbor::Library::open(&config.path)?;

            let entrypoint = unsafe {
                library.symbol::<hhaven_plugin::PluginEntrypoint>(PLUGIN_ENTRYPOINT_NAME)?
            };

            let plugin = entrypoint(Box::new(PluginConfig {
                config: config.config,
            }));

            tracing::info!(
                "Loaded plugin: {}, version: {}",
                config.name,
                plugin.version
            );

            plugins.insert(config.name.clone(), plugin);
            _librarys.insert(config.name, library);
        }

        Ok(Self { plugins, _librarys })
    }

    pub fn outgoing(&self, name: &str) -> Result<&Box<dyn Outgoing>> {
        let plugin = self
            .plugins
            .get(name)
            .ok_or(anyhow::anyhow!("Plugin not found"))?;

        Ok(plugin
            .outgoing
            .as_ref()
            .ok_or(anyhow::anyhow!("Outgoing not found"))?)
    }
}
