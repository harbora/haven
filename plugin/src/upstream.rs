use std::net::IpAddr;

use anyhow::Result;

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
    fn connect(&self, host: UpstreamHost) -> Result<Box<dyn Stream>>;

    /// Sends data to the remote host
    ///
    /// # Arguments
    /// * `stream` - The stream to send data to
    /// * `data` - The data to send
    fn send(&self, stream: &dyn Stream, data: &[u8]) -> Result<()>;

    /// Receives data from the remote host
    ///
    /// # Arguments
    /// * `stream` - The stream to receive data from
    /// * `buf` - The buffer to receive data into
    fn recv(&self, stream: &dyn Stream, buf: &mut [u8]) -> Result<()>;
}
