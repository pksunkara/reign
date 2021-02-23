// Completely copied from https://github.com/stephank/hyper-staticfile
// because I need to modify this to implement ranges

use reign_plugin::reign_router::{futures::Stream as FutureStream, hyper::body::Bytes};
use tokio::{
    fs::File,
    io::{AsyncRead, ReadBuf},
};

use std::{
    io::Error,
    mem::MaybeUninit,
    pin::Pin,
    task::{Context, Poll},
};

const BUF_SIZE: usize = 8 * 1024;

/// Wraps a `tokio::fs::File`, and implements a stream of `Bytes`s.
pub(crate) struct Stream {
    file: File,
    buf: Box<[MaybeUninit<u8>; BUF_SIZE]>,
}

impl Stream {
    /// Create a new stream from the given file.
    pub(crate) fn new(file: File) -> Stream {
        let buf = Box::new([MaybeUninit::uninit(); BUF_SIZE]);
        Stream { file, buf }
    }
}

impl FutureStream for Stream {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let Self {
            ref mut file,
            ref mut buf,
        } = *self;

        let mut read_buf = ReadBuf::uninit(&mut buf[..]);

        match Pin::new(file).poll_read(cx, &mut read_buf) {
            Poll::Ready(Ok(())) => {
                let filled = read_buf.filled();
                if filled.is_empty() {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Ok(Bytes::copy_from_slice(filled))))
                }
            }
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}
