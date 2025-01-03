use hhaven_acceptor_tcp::{TcpAcceptor, TcpAcceptorConfig};
use hhaven_core::{Acceptor, MemoryMetadataStorage};

#[tokio::main]
async fn main() {
    let config = TcpAcceptorConfig {
        address: "0.0.0.0:8080".parse().unwrap(),
    };

    let acceptor = TcpAcceptor::new(config).await.unwrap();

    let mut ms = MemoryMetadataStorage::default();

    let mut acceptor = acceptor;

    let _stream = acceptor.accept(&mut ms).await.unwrap();

    println!("{:?}", ms);
}
