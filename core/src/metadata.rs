use std::future::Future;

use anyhow::Result;

pub trait MetadataStorage {
    fn get(&self, key: &str) -> impl Future<Output = Result<String>>;

    fn set(&mut self, key: &str, value: &[u8]) -> impl Future<Output = Result<()>>;
}
