use std::io::Result;
use std::pin::Pin;
use std::process::Stdio;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite, BufReader, BufWriter};
use tokio::process::{Child, ChildStdout, Command};
use tokio_stream::Stream;
use tokio_util::bytes::Bytes;
use tokio_util::io::ReaderStream;
use tokio_util::sync::CancellationToken;

pub mod info;
pub mod video;

fn stdio_unavailable() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::ResourceBusy,
        "unable to get stdio from child process",
    )
}

#[derive(Debug)]
pub struct Transcoder {
    cancellation_token: CancellationToken,
    child: Child,
    reader_task: tokio::task::JoinHandle<Result<u64>>,
    writer_task: tokio::task::JoinHandle<Result<u64>>,
}

impl Transcoder {
    pub fn new<R, W>(mut cmd: Command, reader: R, writer: W) -> Result<Self>
    where
        R: AsyncRead + Send + Unpin + 'static,
        W: AsyncWrite + Send + Unpin + 'static,
    {
        let mut child = cmd
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let cancellation_token = CancellationToken::new();

        let mut child_stdin = child.stdin.take().ok_or_else(stdio_unavailable)?;
        let reader_cancel = cancellation_token.clone();
        let reader_task = tokio::spawn(async move {
            let mut buf = BufReader::new(reader);
            tokio::select! {
                res = tokio::io::copy(&mut buf, &mut child_stdin) => {
                    res
                }
                _ = reader_cancel.cancelled() => {
                    Err(std::io::Error::other("cancelled"))
                }
            }
        });

        let mut child_stdout = child.stdout.take().ok_or_else(stdio_unavailable)?;
        let writer_cancel = cancellation_token.clone();
        let writer_task = tokio::spawn(async move {
            let mut buf = BufWriter::new(writer);
            tokio::select! {
                res = tokio::io::copy(&mut child_stdout, &mut buf) => {
                    res
                }
                _ = writer_cancel.cancelled() => {
                    Err(std::io::Error::other("cancelled"))
                }
            }
        });

        Ok(Transcoder {
            cancellation_token,
            child,
            reader_task,
            writer_task,
        })
    }

    pub fn abort(&self) {
        self.cancellation_token.cancel();
    }

    pub async fn wait(mut self) {
        if self.cancellation_token.is_cancelled() {
            let _ = self.child.start_kill();
            let _ = self.reader_task.await;
            let _ = self.writer_task.await;
        }
        let _ = self.child.wait().await;
    }
}

#[derive(Debug)]
pub struct StreamTranscoder {
    cancellation_token: CancellationToken,
    child: Child,
    reader_task: tokio::task::JoinHandle<Result<u64>>,
    stream: ReaderStream<ChildStdout>,
}

impl StreamTranscoder {
    pub fn new<R>(mut cmd: Command, reader: R) -> Result<Self>
    where
        R: AsyncRead + Send + Unpin + 'static,
    {
        let mut child = cmd
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let cancellation_token = CancellationToken::new();

        let mut child_stdin = child.stdin.take().ok_or_else(stdio_unavailable)?;
        let reader_cancel = cancellation_token.clone();
        let reader_task = tokio::spawn(async move {
            let mut buf = BufReader::new(reader);
            tokio::select! {
                res = tokio::io::copy(&mut buf, &mut child_stdin) => {
                    res
                }
                _ = reader_cancel.cancelled() => {
                    Err(std::io::Error::other("cancelled"))
                }
            }
        });

        let child_stdout = child.stdout.take().ok_or_else(stdio_unavailable)?;
        let stream = ReaderStream::with_capacity(child_stdout, 10 * 1024 * 1024);

        Ok(StreamTranscoder {
            cancellation_token,
            child,
            reader_task,
            stream,
        })
    }
}

impl Stream for StreamTranscoder {
    type Item = Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut();
        Pin::new(&mut this.stream).poll_next(cx)
    }
}

impl Drop for StreamTranscoder {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
        self.cancellation_token.cancel();
        self.reader_task.abort();
    }
}
