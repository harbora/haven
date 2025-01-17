use std::sync::Arc;

use hhaven_plugin::{Plugin, PluginConfig};
use tokio::runtime::Runtime;

use crate::{Config, DirectOutgoing};

pub fn _entry_point(config: Box<PluginConfig>) -> Box<Plugin> {
    let config: Config = config.config.try_into().unwrap();

    let runtime = Runtime::new().expect("Failed to create runtime");

    Box::new(Plugin {
        name: "direct",
        version: "0.1.0",
        outgoing: Some(Box::new(DirectOutgoing::new(config, Arc::new(runtime)))),
    })
}

#[no_mangle]
pub extern "C" fn hh_plugin_entrypoint(config: Box<PluginConfig>) -> Box<Plugin> {
    _entry_point(config)
}
