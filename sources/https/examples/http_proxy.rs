use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use hhaven_core::{Authenticator, Forwarder, Source};
use hhaven_source_https::{Config, InnerConfig, Server};
use tokio::io::{AsyncRead, AsyncWrite};

struct Auth;

impl Authenticator for Auth {
    fn authenticate(&self, _token: &str) -> Result<u64> {
        Ok(2)
    }
}

struct Fder;

impl Forwarder for Fder {
    async fn forward(
        &self,
        _source: Source,
        _receiver_part: impl AsyncWrite + AsyncRead + Send,
    ) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config {
        certs_path: PathBuf::from("./certs/sample.pem"),
        private_key_path: PathBuf::from("./certs/sample.rsa"),
        port: 8080,
        inner: InnerConfig {
            upstream_header: None,
        },
    };

    let server = Server::new(config, Arc::new(Auth), Arc::new(Fder))
        .await
        .unwrap();

    server.run().await.unwrap();
}
