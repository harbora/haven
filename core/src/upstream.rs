use std::net::IpAddr;

use crate::Stream;

/// Represents a remote host that can be connected to
pub enum UpstreamHost {
    /// An IP address (IPv4 or IPv6)
    IpAddr(IpAddr),
    /// A domain name
    Domain(String),
}

/// A trait for implementing upstream connection handlers
///
/// This trait is implemented by plugins to provide upstream connection functionality.
/// It is used to establish connections to remote hosts.
pub trait Upstream {
    /// Connects to a remote host using the provided stream
    ///
    /// # Arguments
    /// * `host` - The remote host to connect to
    /// * `stream` - The stream to use for the connection
    fn connect(&self, host: UpstreamHost, stream: &dyn Stream);
}
