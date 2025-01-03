use anyhow::Result;
use hhaven_core::{Acceptor, MetadataStorage, Stream};
use tokio::net::TcpListener;

use crate::{TcpAcceptorConfig, TcpStreamWapper};

pub struct TcpAcceptor {
    listener: TcpListener,
}

impl Acceptor for TcpAcceptor {
    type Config = TcpAcceptorConfig;

    async fn new(config: Self::Config) -> Result<Self> {
        let listener = TcpListener::bind(config.address).await?;

        Ok(Self { listener })
    }

    async fn accept(&mut self, metadata: &mut impl MetadataStorage) -> Result<impl Stream> {
        let (stream, addr) = self.listener.accept().await?;

        let addr_ip = format!("{}", addr.ip());
        let addr_port = format!("{}", addr.port());

        log::info!("accept connection from {}", addr);

        metadata.set("tcp_acceptor", "ip", "addr", &addr_ip).await?;
        metadata
            .set("tcp_acceptor", "port", "addr", &addr_port)
            .await?;

        Ok(TcpStreamWapper(stream))
    }
}
