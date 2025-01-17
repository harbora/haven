use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use anyhow::Result;
use async_trait::async_trait;
use hhaven_plugin::{Outgoing, OutgoingHost, OutgoingUdp, Stream};
use tokio::net::{TcpSocket, UdpSocket};

use crate::{Config, DirectUdpSocket};

pub struct DirectOutgoing {
    interface: Option<String>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl DirectOutgoing {
    pub fn new(config: Config, runtime: Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            interface: config.interface,
            runtime,
        }
    }
}

#[async_trait]
impl Outgoing for DirectOutgoing {
    async fn connect(&self, host: OutgoingHost) -> Result<Box<dyn Stream>> {
        let interface = self.interface.clone();

        self.runtime.spawn(_connect(host, interface)).await?
    }

    async fn udp(&self, host: OutgoingHost) -> Result<Box<dyn OutgoingUdp>> {
        let addr = parse_ip(host).await?;

        let socket = UdpSocket::bind(addr).await?;

        if let Some(interface) = &self.interface {
            socket.bind_device(Some(interface.as_bytes()))?;
        }

        Ok(Box::new(DirectUdpSocket(socket)))
    }
}

async fn _connect(host: OutgoingHost, interface: Option<String>) -> Result<Box<dyn Stream>> {
    let addr = parse_ip(host).await?;

    let socket = match addr.ip() {
        IpAddr::V4(_) => TcpSocket::new_v4()?,
        IpAddr::V6(_) => TcpSocket::new_v6()?,
    };

    if let Some(interface) = interface {
        socket.bind_device(Some(interface.as_bytes()))?;
    }

    let stream = socket.connect(addr).await?;

    Ok(Box::new(stream))
}

async fn parse_ip(host: OutgoingHost) -> Result<SocketAddr> {
    match host {
        OutgoingHost::IpAddr(ip, port) => Ok(SocketAddr::new(ip, port)),
        OutgoingHost::Domain(domain, port) => {
            let addr = tokio::net::lookup_host(format!("{}:{}", domain, port)).await?;
            Ok(SocketAddr::new(
                addr.into_iter()
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("no address found"))?
                    .ip(),
                port,
            ))
        }
    }
}
