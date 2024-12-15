use std::path::PathBuf;

use hhaven_entrypoint::Config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config {
        certs_path: PathBuf::from("./certs/sample.pem"),
        private_key_path: PathBuf::from("./certs/sample.rsa"),
        port: 8080,
    };

    let server = hhaven_entrypoint::Server::new(config).await.unwrap();

    server.run().await.unwrap();
}
