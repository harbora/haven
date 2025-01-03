use std::sync::Arc;

use anyhow::Result;
use hhaven_core::{Inbound, MetadataStorage, Stream};
use tokio_rustls::{
    rustls::{server::ResolvesServerCertUsingSni, sign::CertifiedKey, ServerConfig},
    TlsAcceptor,
};

use crate::{utils, TLSInboundConfig, TLSInboundStream};

pub struct TLSInbound {
    acceptor: TlsAcceptor,
}

impl Inbound for TLSInbound {
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

        let stream = self.acceptor.accept(stream).await?;

        let (_, server_conn) = stream.get_ref();

        let sni = server_conn.server_name().map(|f| f.as_bytes().to_vec());
        let alpn = server_conn.alpn_protocol().map(|f| f.to_vec());

        let info = ConnectionInfo { sni, alpn };

        info.set_metadata(metadata).await?;

        let stream = TLSInboundStream { stream };

        Ok(stream)
    }
}

#[derive(Debug)]
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
