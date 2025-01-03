use std::net::{IpAddr, SocketAddr};

use anyhow::Result;
use hhaven_core::{Acceptor, MetadataStorage, Stream};
use tokio::net::TcpListener;

use crate::{TcpAcceptorConfig, TcpStreamWapper};

pub struct TcpAcceptor {
    listener: TcpListener,

    addr: SocketAddr,
}

impl Acceptor for TcpAcceptor {
    type Config = TcpAcceptorConfig;

    async fn new(config: Self::Config) -> Result<Self> {
        let listener = TcpListener::bind(config.address).await?;

        Ok(Self {
            listener,
            addr: config.address,
        })
    }

    async fn accept(&mut self, metadata: &mut impl MetadataStorage) -> Result<impl Stream> {
        let (stream, addr) = self.listener.accept().await?;

        log::info!("accept connection from {}", addr);

        let (ip, version) = ip_bytes(addr.ip());
        metadata.set("tcp_acceptor.from.ip", &ip).await?;
        metadata
            .set("tcp_acceptor.from.ip_version", &[version])
            .await?;
        metadata
            .set("tcp_acceptor.from.port", &port_bytes(addr.port()))
            .await?;

        self.set_to(metadata).await?;

        Ok(TcpStreamWapper(stream))
    }
}

impl TcpAcceptor {
    async fn set_to(&self, metadata: &mut impl MetadataStorage) -> Result<()> {
        let (ip, version) = ip_bytes(self.addr.ip());

        metadata.set("tcp_acceptor.to.ip", &ip).await?;
        metadata
            .set("tcp_acceptor.to.ip_version", &[version])
            .await?;
        metadata
            .set("tcp_acceptor.to.port", &port_bytes(self.addr.port()))
            .await?;

        Ok(())
    }
}

pub fn ip_bytes(ip: IpAddr) -> (Vec<u8>, u8) {
    match ip {
        IpAddr::V4(ip) => (ip.octets().to_vec(), 4),
        IpAddr::V6(ip) => (ip.octets().to_vec(), 6),
    }
}

pub fn port_bytes(port: u16) -> Vec<u8> {
    port.to_le_bytes().to_vec()
}
