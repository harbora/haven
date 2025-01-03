use std::collections::BTreeMap;

use hhaven_acceptor_tcp::{TcpAcceptor, TcpAcceptorConfig};
use hhaven_core::{Acceptor, Inbound, MemoryMetadataStorage};
use hhaven_inbound_tls::{CertKey, TLSInbound, TLSInboundConfig};

#[tokio::main]
async fn main() {
    let mut ms = MemoryMetadataStorage::default();

    let config = TcpAcceptorConfig {
        address: "0.0.0.0:8080".parse().unwrap(),
    };

    let acceptor = TcpAcceptor::new(config).await.unwrap();

    let mut acceptor = acceptor;

    let tcp_stream = acceptor.accept(&mut ms).await.unwrap();

    let cert_key = CertKey {
        cert: include_bytes!("../certs/sample.pem").to_vec(),
        key: include_bytes!("../certs/sample.rsa").to_vec(),
    };

    let mut config = TLSInboundConfig {
        cert_key: BTreeMap::new(),
        alpn_protocols: vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()],
    };

    config
        .cert_key
        .insert("testserver.com".to_string(), cert_key);

    let mut inbound = TLSInbound::new(config).await.unwrap();

    let _tls_stream = inbound.warp(tcp_stream, &mut ms).await.unwrap();

    println!("{:?}", ms);
}
