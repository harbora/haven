/// A stream that can be used to communicate with a remote host
///
/// This trait is automatically implemented for any type that implements `Send + Sync + Unpin`.
/// It is used by the [`Upstream`](crate::Upstream) trait to handle connections.
pub trait Stream: Send + Sync + Unpin {}

impl<T: Send + Sync + Unpin> Stream for T {}
