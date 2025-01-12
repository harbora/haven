pub struct BindConfig {
    pub addr: String,
    pub device: Option<String>,
}

pub struct Config {
    pub bind: Vec<BindConfig>,
}
