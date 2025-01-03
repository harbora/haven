use std::future::Future;

use anyhow::Result;

pub trait MetadataStorage {
    fn get(&self, namespace: &str, prefix: &str, key: &str)
        -> impl Future<Output = Result<String>>;

    fn set(
        &mut self,
        namespace: &str,
        prefix: &str,
        key: &str,
        value: &str,
    ) -> impl Future<Output = Result<()>>;
}
