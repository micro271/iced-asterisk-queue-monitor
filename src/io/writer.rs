use std::{io::Error, pin::Pin};

use bytes::{Buf, BytesMut};
use tokio::io::AsyncWrite;

pub struct BufWriter<T> {
    inner: T,
    buffer: BytesMut,
}

impl<T: AsyncWrite> BufWriter<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: BytesMut::new(),
        }
    }

    pub fn to_write(&mut self, slice: &[u8]) {
        self.buffer.extend_from_slice(slice);
    }
}

impl<T: AsyncWrite> AsyncWrite for BufWriter<T> {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        if self.buffer.is_empty() {
            return std::task::Poll::Ready(Err(Error::new(std::io::ErrorKind::InvalidInput, "Buffer is empty")));
        }

        let this = unsafe {Pin::get_unchecked_mut(self)};
        let wr = unsafe {Pin::new_unchecked(&mut this.inner)};
        let n = futures::ready!(wr.poll_write(cx, buf))?;
        this.buffer.advance(n);
        std::task::Poll::Ready(Ok(n))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {

        if self.buffer.is_empty() {
            return std::task::Poll::Ready(Err(Error::new(std::io::ErrorKind::InvalidInput, "Buffer is empty")));
        }

        let this = unsafe { Pin::get_unchecked_mut(self) };
        let wr = unsafe {Pin::new_unchecked(&mut this.inner)};
        let n = futures::ready!(wr.poll_write(cx, &mut this.buffer))?;

        if n != this.buffer.len() {
            this.buffer.advance(n);
            std::task::Poll::Pending
        } else {
            this.buffer.clear();
            std::task::Poll::Ready(Ok(()))
        }
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        
        if self.buffer.is_empty() {
            return std::task::Poll::Ready(Err(Error::new(std::io::ErrorKind::InvalidInput, "Buffer is empty")));
        }

        futures::ready!(self.as_mut().poll_flush(cx))?;

        unsafe { Pin::map_unchecked_mut(self, |x| &mut x.inner) }.poll_shutdown(cx)
    }
}
