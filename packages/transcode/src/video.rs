use tokio::io::{AsyncRead, AsyncWrite};
use tokio::process::Command;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Format {
    #[serde(rename = "mp4")]
    Mp4,
}

impl Format {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mp4 => "mp4",
        }
    }

    pub fn as_mime(&self) -> &'static str {
        match self {
            Self::Mp4 => "video/mp4",
        }
    }
}

#[derive(Debug)]
pub struct Builder {
    format: Format,
}

impl Builder {
    pub fn new(format: Format) -> Self {
        Self { format }
    }

    fn with_args<'a, 'b>(&'a self, cmd: &'b mut Command) -> &'b mut Command
    where
        'b: 'a,
    {
        match self.format {
            Format::Mp4 => cmd
                .arg("-f")
                .arg("mp4")
                .arg("-c")
                .arg("copy")
                .arg("-movflags")
                .arg("frag_keyframe+empty_moov"),
        }
    }

    fn command(&self) -> Command {
        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y") // overwrite existing files
            .arg("-i")
            .arg("pipe:0"); // input from stdin
        self.with_args(&mut cmd);
        cmd.arg("pipe:1"); // output to stdout
        cmd
    }

    pub fn build_transcoder(
        &self,
        reader: impl AsyncRead + Unpin + Send + 'static,
        writer: impl AsyncWrite + Unpin + Send + 'static,
    ) -> std::io::Result<crate::Transcoder> {
        crate::Transcoder::new(self.command(), reader, writer)
    }

    pub fn build_stream_transcoder(
        &self,
        reader: impl AsyncRead + Unpin + Send + 'static,
    ) -> std::io::Result<super::StreamTranscoder> {
        crate::StreamTranscoder::new(self.command(), reader)
    }
}

#[cfg(test)]
mod tests {
    // requires to download https://download.blender.org/peach/bigbuckbunny_movies/BigBuckBunny_640x360.m4v
    // and store it in the .tmp directory

    use any_storage::{Store, StoreFile, StoreMetadata, WriteOptions};

    #[tokio::test]
    async fn should_transcode() {
        let storage = any_storage::local::LocalStore::new(".tmp");
        let source_file = storage.get_file("BigBuckBunny_640x360.m4v").await.unwrap();
        let source_reader = source_file.read(..).await.unwrap();
        //
        let storage = any_storage::local::LocalStore::new(".tmp");
        let target_file = storage.get_file("BigBuckBunny_640x360.mp4").await.unwrap();
        let target_writer = target_file.write(WriteOptions::create()).await.unwrap();
        //
        let transcoder = super::Builder::new(super::Format::Mp4)
            .build_transcoder(source_reader, target_writer)
            .unwrap();
        transcoder.wait().await;
        //
        let meta = target_file.metadata().await.unwrap();
        assert_ne!(meta.size(), 0);
    }
}
