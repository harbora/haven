use std::future::Future;

use anyhow::Result;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::Source;

pub trait Forwarder: Send + Sync + 'static {
    fn forward(
        &self,
        source: Source,
        receiver_part: impl AsyncWrite + AsyncRead + Send,
    ) -> impl Future<Output = Result<()>> + Send;
}
