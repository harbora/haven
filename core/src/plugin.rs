use crate::Upstream;

/// A plugin that provides upstream functionality
pub struct Plugin {
    /// Name of the plugin
    pub name: &'static str,
    /// Version of the plugin
    pub version: &'static str,
    /// The upstream implementation provided by this plugin
    pub upstream: Option<Box<dyn Upstream>>,
}

pub struct PluginConfig {
    pub config: String,
}

/// Entry point function that plugins must export
///
/// This function is called when loading the plugin to get the plugin instance.
/// It must be exported with the name "hh_plugin_entrypoint".
pub type PluginEntrypoint = extern "C" fn(config: Box<PluginConfig>) -> Box<Plugin>;
