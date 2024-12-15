use std::path::PathBuf;

pub struct Config {
    pub port: u16,
    pub certs_path: PathBuf,
    pub private_key_path: PathBuf,
}
