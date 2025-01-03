use tokio::io::{AsyncRead, AsyncWrite};

pub trait Stream: AsyncRead + AsyncWrite + Send + Sync + Sized + 'static {}

// pub trait Stream = AsyncRead + AsyncWrite + Send + Sync + Sized + 'static;
