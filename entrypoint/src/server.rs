use std::sync::Arc;

use anyhow::Result;
use http_body_util::Empty;
use hyper::{body::Bytes, service::service_fn, Request, Response};
use hyper_util::{rt::TokioExecutor, server::conn};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{rustls::ServerConfig, TlsAcceptor};

use crate::{utils, Config};

pub struct Server {
    http_1_2_listener: TcpListener,

    tls_acceptor: TlsAcceptor,
}

impl Server {
    pub async fn new(config: Config) -> Result<Self> {
        let certs = utils::load_certs(&config.certs_path).await?;
        let private_key = utils::load_private_key(&config.private_key_path).await?;

        let mut server_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        server_config.alpn_protocols =
            vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];

        let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));

        log::info!("Listening on port [::]:{}", config.port);

        Ok(Self {
            http_1_2_listener: TcpListener::bind(("::", config.port)).await?,

            tls_acceptor,
        })
    }

    pub async fn run(self) -> Result<()> {
        let tls_acceptor = self.tls_acceptor.clone();

        loop {
            let res = self.http_1_2_listener.accept().await;
            if let Ok((stream, addr)) = res {
                log::info!("Accepted connection from {}", addr);

                handle_connection(stream, tls_acceptor.clone());
            } else {
                log::error!("Failed to accept connection: {:?}", res.err());
            }
        }
    }
}

fn handle_connection(stream: TcpStream, tls_acceptor: TlsAcceptor) {
    tokio::spawn(async move {
        let stream = tls_acceptor.accept(stream).await;

        if let Ok(stream) = stream {
            log::info!("Accepted TLS connection");

            let io = hyper_util::rt::TokioIo::new(stream);

            let res = conn::auto::Builder::new(TokioExecutor::new())
                .http1()
                .http2()
                .serve_connection_with_upgrades(io, service_fn(proxy))
                .await;

            if let Err(e) = res {
                log::error!("Failed to serve connection: {:?}", e);
            }
        } else {
            log::error!("Failed to accept TLS connection: {:?}", stream.err());
            return;
        }
    });
}

async fn proxy(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Empty<Bytes>>, hyper::Error> {
    println!("{:?}", req);

    let res = Empty::<Bytes>::new();

    Ok(Response::new(res))
}
