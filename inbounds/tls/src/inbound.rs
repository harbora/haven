use std::sync::Arc;

use anyhow::Result;
use hhaven_core::{Inbound, MetadataStorage, Stream};
use tokio_rustls::{
    rustls::{server::ResolvesServerCertUsingSni, sign::CertifiedKey, ServerConfig},
    TlsAcceptor,
};

use crate::{utils, TLSInboundConfig, TLSInboundStream};

pub struct TLSInboundAcceptor {
    acceptor: TlsAcceptor,
}

impl Inbound for TLSInboundAcceptor {
    type Config = TLSInboundConfig;

    async fn new(config: Self::Config) -> Result<Self> {
        let builder = ServerConfig::builder().with_no_client_auth();

        let key_provider = builder.crypto_provider().key_provider;

        let mut cert_resolver = ResolvesServerCertUsingSni::new();

        for (sni, cert_key) in config.cert_key {
            let certs = utils::load_certs(&cert_key.cert)?;
            let key = utils::load_private_key(&cert_key.key)?;

            let key = key_provider.load_private_key(key)?;

            let ck = CertifiedKey::new(certs, key);

            cert_resolver.add(&sni, ck)?;
        }

        let mut server_config = builder.with_cert_resolver(Arc::new(cert_resolver));

        server_config.alpn_protocols = config.alpn_protocols;

        let acceptor = TlsAcceptor::from(Arc::new(server_config));

        Ok(Self { acceptor })
    }

    async fn warp(
        &mut self,
        stream: impl Stream,
        metadata: &mut impl MetadataStorage,
    ) -> Result<impl Stream> {
        let stream: Box<dyn Stream> = Box::new(stream);

        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

        let stream = self
            .acceptor
            .accept_with(stream, |conn| {
                let sni = conn.server_name();
                let alpn = conn.alpn_protocol();

                let info = ConnectionInfo {
                    sni: sni.map(|f| f.as_bytes().to_vec()),
                    alpn: alpn.map(|f| f.to_vec()),
                };

                let _ = sender.send(info);
            })
            .await?;

        if let Some(info) = receiver.recv().await {
            info.set_metadata(metadata).await?;
        } else {
            log::error!("connection info not found");
        }

        let stream = TLSInboundStream { stream };

        Ok(stream)
    }
}

struct ConnectionInfo {
    sni: Option<Vec<u8>>,
    alpn: Option<Vec<u8>>,
}

impl ConnectionInfo {
    pub async fn set_metadata(self, metadata: &mut impl MetadataStorage) -> Result<()> {
        if let Some(sni) = self.sni {
            metadata.set("tls-inbound.sni", &sni).await?;
        }

        if let Some(alpn) = self.alpn {
            metadata.set("tls-inbound.alpn", &alpn).await?;
        }

        Ok(())
    }
}
