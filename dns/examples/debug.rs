use std::env;

use haven_dns::{BindConfig, Config, HavenDNSServer, Resolver};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database = env::var("DATABASE_URL").unwrap();

    let config = Config {
        bind: vec![BindConfig {
            addr: "[::]:5300".to_string(),
            device: None,
        }],
        database,
        resolver: vec![Resolver::UDP("114.114.114.114:53".parse().unwrap())],
    };

    let mut server = HavenDNSServer::new(config).await.unwrap();
    server.run().await.unwrap();

    tokio::signal::ctrl_c().await.unwrap();

    server.stop().await.unwrap();
}
