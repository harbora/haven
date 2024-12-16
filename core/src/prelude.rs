use anyhow::Result;

pub trait Authenticator: Send + Sync + 'static {
    fn authenticate(&self, token: &str) -> Result<u64>;
}
