use std::{collections::HashMap, future::Future};

use anyhow::Result;

pub trait MetadataStorage {
    fn get(&self, key: &str) -> impl Future<Output = Result<Vec<u8>>>;

    fn set(&mut self, key: &str, value: &[u8]) -> impl Future<Output = Result<()>>;
}

#[derive(Default, Debug)]
pub struct MemoryMetadataStorage {
    data: HashMap<String, Vec<u8>>,
}

impl MetadataStorage for MemoryMetadataStorage {
    async fn get(&self, key: &str) -> Result<Vec<u8>> {
        let res = self.data.get(key).ok_or(anyhow::anyhow!("key not found"))?;

        Ok(res.clone())
    }

    async fn set(&mut self, key: &str, value: &[u8]) -> Result<()> {
        self.data.insert(key.to_string(), value.to_vec());
        Ok(())
    }
}
