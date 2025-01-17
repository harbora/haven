use crate::Outgoing;

/// A plugin that provides upstream functionality
pub struct Plugin {
    /// Name of the plugin
    pub name: &'static str,
    /// Version of the plugin
    pub version: &'static str,
    /// The upstream implementation provided by this plugin
    pub outgoing: Option<Box<dyn Outgoing>>,
}

pub struct PluginConfig {
    pub config: toml::Value,
}

/// Entry point function that plugins must export
///
/// This function is called when loading the plugin to get the plugin instance.
/// It must be exported with the name "hh_plugin_entrypoint".
pub type PluginEntrypoint = extern "C" fn(config: Box<PluginConfig>) -> Box<Plugin>;

pub const PLUGIN_ENTRYPOINT_NAME: &str = "hh_plugin_entrypoint";
