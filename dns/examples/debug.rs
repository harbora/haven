use haven_dns::{BindConfig, Config, HavenDNSServer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config {
        bind: vec![BindConfig {
            addr: "[::]:5300".to_string(),
            device: None,
        }],
    };

    let mut server = HavenDNSServer::new(config).await.unwrap();
    server.run().await.unwrap();

    tokio::signal::ctrl_c().await.unwrap();

    server.stop().await.unwrap();
}
