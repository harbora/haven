use std::net::IpAddr;

use anyhow::Result;
use async_trait::async_trait;

use crate::Stream;

/// Represents a remote host that can be connected to
pub enum OutgoingHost {
    /// An IP address (IPv4 or IPv6)
    IpAddr(IpAddr, u16),
    /// A domain name
    Domain(String, u16),
}

/// A trait for implementing upstream connection handlers
///
/// This trait is implemented by plugins to provide upstream connection functionality.
/// It is used to establish connections to remote hosts.
#[async_trait]
pub trait Outgoing {
    /// Connects to a remote host using the provided stream
    ///
    /// # Arguments
    /// * `host` - The remote host to connect to
    async fn connect(&self, host: OutgoingHost) -> Result<Box<dyn Stream>>;

    async fn udp(&self, host: OutgoingHost) -> Result<Box<dyn OutgoingUdp>>;
}

#[async_trait]
pub trait OutgoingUdp {
    /// Sends data to the remote host
    ///
    /// # Arguments
    /// * `data` - The data to send
    async fn send(&self, data: &[u8]) -> Result<()>;

    /// Receives data from the remote host
    ///
    /// # Arguments
    /// * `buf` - The buffer to receive data into
    async fn recv(&self, buf: &mut [u8]) -> Result<()>;
}
