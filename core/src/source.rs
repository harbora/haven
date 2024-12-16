use std::{fmt::Display, net::SocketAddr};

#[derive(Debug, Clone)]
pub struct Source {
    pub id: u64,
    pub addr: SocketAddr,
    pub ty: SourceType,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Source {{ id: {}, addr: {}, ty: {:?} }}",
            self.id, self.addr, self.ty
        )
    }
}

#[derive(Debug, Clone)]
pub enum SourceType {
    HttpProxy {
        upstream_domain: String,
        upstream_port: u16,
        http_version: http::Version,
    },
    DNSLegacy,
}
