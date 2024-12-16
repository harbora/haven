use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub certs_path: PathBuf,
    pub private_key_path: PathBuf,

    pub inner: InnerConfig,
}

#[derive(Clone)]
pub struct InnerConfig {
    pub upstream_header: Option<String>,
}
