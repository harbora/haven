use std::time::Duration;

use anyhow::Result;
use hickory_server::ServerFuture;
use tokio::net::UdpSocket;

use crate::{CachedMatcher, Config, HavenDNSHandler, LocalResolver};

pub struct HavenDNSServer {
    server: ServerFuture<HavenDNSHandler>,
}

impl HavenDNSServer {
    pub async fn new(config: Config) -> Result<Self> {
        let db_pool = sqlx::postgres::PgPool::connect(&config.database).await?;

        let matcher = CachedMatcher::new(1000, (), Duration::from_secs(300));

        let local_resolver = LocalResolver::new(db_pool).await;

        let mut server = ServerFuture::new(HavenDNSHandler::new(matcher, local_resolver));

        for bind in config.bind {
            let udp_socket = UdpSocket::bind(bind.addr).await?;

            if let Some(device) = bind.device {
                udp_socket.bind_device(Some(device.as_bytes()))?;
            }

            server.register_socket(udp_socket);
        }

        Ok(Self { server })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.server.block_until_done().await?;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        self.server.shutdown_gracefully().await?;

        Ok(())
    }
}
