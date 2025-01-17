use anyhow::Result;
use async_trait::async_trait;
use hhaven_plugin::OutgoingUdp;

pub struct DirectUdpSocket(pub tokio::net::UdpSocket);

#[async_trait]
impl OutgoingUdp for DirectUdpSocket {
    async fn send(&self, buf: &[u8]) -> Result<()> {
        self.0.send(buf).await?;

        Ok(())
    }

    async fn recv(&self, buf: &mut [u8]) -> Result<()> {
        self.0.recv(buf).await?;

        Ok(())
    }
}
