use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub path: PathBuf,
    pub config: toml::Value,
}
