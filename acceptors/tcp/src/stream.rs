use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

use hhaven_core::Stream;
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::TcpStream,
};

pub struct TcpStreamWapper(pub(crate) TcpStream);

impl AsyncRead for TcpStreamWapper {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        let this = self.get_mut();

        let mut stream = &mut this.0;

        let pinned = std::pin::pin!(stream);

        pinned.poll_read(cx, buf)
    }
}

impl AsyncWrite for TcpStreamWapper {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        let this = self.get_mut();

        let mut stream = &mut this.0;

        let pinned = std::pin::pin!(stream);

        pinned.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();

        let mut stream = &mut this.0;

        let pinned = std::pin::pin!(stream);

        pinned.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();

        let mut stream = &mut this.0;

        let pinned = std::pin::pin!(stream);

        pinned.poll_shutdown(cx)
    }
}

impl Stream for TcpStreamWapper {}
