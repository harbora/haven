use std::net::SocketAddr;

pub struct BindConfig {
    pub addr: String,
    pub device: Option<String>,
}

pub enum Resolver {
    UDP(SocketAddr),
    DoT(SocketAddr),
    DoH(String),
}

pub struct Config {
    pub bind: Vec<BindConfig>,
    pub database: String,
    pub resolver: Vec<Resolver>,
}
