use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use hhaven_core::Stream;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio_rustls::server::TlsStream;

pub struct TLSInboundStream {
    pub(crate) stream: TlsStream<Box<dyn Stream>>,
}

impl AsyncRead for TLSInboundStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.get_mut();

        let mut stream = &mut this.stream;

        let pinned = std::pin::pin!(stream);

        pinned.poll_read(cx, buf)
    }
}

impl AsyncWrite for TLSInboundStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();

        let mut stream = &mut this.stream;

        let pinned = std::pin::pin!(stream);

        pinned.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();

        let mut stream = &mut this.stream;

        let pinned = std::pin::pin!(stream);

        pinned.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();

        let mut stream = &mut this.stream;

        let pinned = std::pin::pin!(stream);

        pinned.poll_shutdown(cx)
    }
}

impl Stream for TLSInboundStream {}
