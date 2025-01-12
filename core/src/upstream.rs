use std::net::IpAddr;

pub enum UpstreamHost {
    IpAddr(IpAddr),
    Domain(String),
}

pub trait Upstream {
    fn connect(&self, host: UpstreamHost);
}
