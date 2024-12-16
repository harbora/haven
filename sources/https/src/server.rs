use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use anyhow::Result;
use hhaven_core::{Authenticator, Forwarder, Source, SourceType};
use http_body_util::{combinators::BoxBody, BodyExt, Empty};
use hyper::{
    body::{Bytes, Incoming},
    service::service_fn,
    Method, Request, Response,
};
use hyper_util::{rt::TokioExecutor, server::conn};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{rustls::ServerConfig, TlsAcceptor};

use crate::{utils, Config, InnerConfig};

pub struct Server<A, F> {
    http_1_2_listener: TcpListener,

    tls_acceptor: TlsAcceptor,

    auth: Arc<A>,

    forwarder: Arc<F>,

    config: InnerConfig,
}

impl<A, F> Server<A, F> {
    pub async fn new(config: Config, auth: Arc<A>, forwarder: Arc<F>) -> Result<Self> {
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

            auth,
            forwarder,
            config: config.inner,
        })
    }
}
impl<A, F> Server<A, F>
where
    A: Authenticator,
    F: Forwarder,
{
    pub async fn run(self) -> Result<()> {
        let tls_acceptor = self.tls_acceptor.clone();

        loop {
            let res = self.http_1_2_listener.accept().await;
            if let Ok((stream, addr)) = res {
                log::info!("Accepted connection from {}", addr);

                handle_connection(
                    stream,
                    tls_acceptor.clone(),
                    addr,
                    self.auth.clone(),
                    self.forwarder.clone(),
                    self.config.clone(),
                );
            } else {
                log::error!("Failed to accept connection: {:?}", res.err());
            }
        }
    }
}

fn handle_connection<A, F>(
    stream: TcpStream,
    tls_acceptor: TlsAcceptor,
    from: SocketAddr,
    auth: Arc<A>,
    forwarder: Arc<F>,
    config: InnerConfig,
) where
    A: Authenticator,
    F: Forwarder,
{
    tokio::spawn(async move {
        let stream = tls_acceptor.accept(stream).await;

        if let Ok(stream) = stream {
            let io = hyper_util::rt::TokioIo::new(stream);

            let res = conn::auto::Builder::new(TokioExecutor::new())
                .http1()
                .http2()
                .serve_connection_with_upgrades(
                    io,
                    service_fn(move |req| {
                        proxy(req, from, auth.clone(), forwarder.clone(), config.clone())
                    }),
                )
                .await;

            if let Err(e) = res {
                log::error!("Failed to serve connection: {:?}", e);
            }
        } else {
            // TODO: Handle HTTP
            log::error!("Failed to accept TLS connection: {:?}", stream.err());
            return;
        }
    });
}

async fn proxy<A, F>(
    req: Request<Incoming>,
    from: SocketAddr,
    auth: Arc<A>,
    forwarder: Arc<F>,
    config: InnerConfig,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>>
where
    A: Authenticator,
    F: Forwarder,
{
    let res = _proxy(req, from, auth, forwarder, config).await;

    match res {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("Failed to proxy request: {:?}", e);
            Err(e)
        }
    }
}

async fn _proxy<A, F>(
    req: Request<Incoming>,
    from: SocketAddr,
    auth: Arc<A>,
    forwarder: Arc<F>,
    config: InnerConfig,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>>
where
    A: Authenticator,
    F: Forwarder,
{
    let source = build_source(&req, from.port(), auth, config, from)?;

    log::info!("Captured connection from {}", source);

    tokio::spawn(async move {
        match hyper::upgrade::on(req).await {
            Ok(upgraded) => {
                let io = hyper_util::rt::TokioIo::new(upgraded);

                if let Err(e) = forwarder.forward(source, io).await {
                    log::error!("Failed to forward connection: {:?}", e);
                }
            }
            Err(e) => {
                log::error!("Failed to upgrade to proxy channel: {:?}", e);
            }
        }
    });

    let res = Empty::<Bytes>::new().map_err(|never| match never {});
    Ok(Response::new(BoxBody::new(res)))
}

fn build_source<A>(
    req: &Request<Incoming>,
    port: u16,
    auth: Arc<A>,
    config: InnerConfig,
    addr: SocketAddr,
) -> Result<Source>
where
    A: Authenticator,
{
    if req.method() == Method::CONNECT {
        let source_addr = if let Some(upstream_header) = config.upstream_header {
            let source_ip = req
                .headers()
                .get(&upstream_header)
                .ok_or(anyhow::anyhow!("{} header not found", upstream_header))?;
            let source_ip = source_ip.to_str()?;
            let source_ip = source_ip.parse::<IpAddr>()?;
            (source_ip, port).into()
        } else {
            addr
        };

        let upstream = req.uri();
        let host = upstream.host().ok_or(anyhow::anyhow!("Host not found"))?;
        let port = upstream.port_u16().unwrap_or(443);

        let http_version = req.version();

        let token = req
            .headers()
            .get("proxy-authorization")
            .ok_or(anyhow::anyhow!("Authorization header not found"))?;
        let token = token.to_str()?;
        let id = auth.authenticate(token)?;

        Ok(Source {
            id,
            addr: source_addr,
            ty: SourceType::HttpProxy {
                upstream_domain: host.into(),
                upstream_port: port,
                http_version,
            },
        })
    } else {
        Err(anyhow::anyhow!("Method not supported"))
    }
}
