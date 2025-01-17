use std::path::PathBuf;

use hhaven_loader::{Config, PluginLoader};
use hhaven_plugin::OutgoingHost;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let loader = PluginLoader::new(vec![Config {
        name: "direct".to_string(),
        path: PathBuf::from("target/release/libhhaven_plugin_direct.so"),
        config: toml::Value::Table(Default::default()),
    }])
    .unwrap();

    let outgoing = loader.outgoing("direct").unwrap();

    let mut stream = outgoing
        .connect(OutgoingHost::Domain("httpbin.org".to_string(), 80))
        .await
        .unwrap();

    stream
        .write_all(
            b"GET /get HTTP/1.1\r\nHost: httpbin.org\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\nConnection: close\r\n\r\n",
        )
        .await
        .unwrap();

    let mut buf = String::new();
    stream.read_to_string(&mut buf).await.unwrap();

    println!("{}", buf);
}
