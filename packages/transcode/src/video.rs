use tokio::io::{AsyncRead, AsyncWrite};
use tokio::process::Command;

#[derive(Debug)]
pub struct Transcoder(crate::Transcoder);

impl Transcoder {
    pub async fn new(
        reader: impl AsyncRead + Unpin + Send + 'static,
        writer: impl AsyncWrite + Unpin + Send + 'static,
    ) -> std::io::Result<Self> {
        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y") // overwrite existing files
            .arg("-i")
            .arg("pipe:0") // input from stdin
            .arg("-vf")
            .arg("pad=iw:ceil(ih/2)*2")
            .arg("-f")
            .arg("mp4") // output format
            .arg("-c:v")
            .arg("libx264")
            .arg("-c:a")
            .arg("aac")
            .arg("-movflags")
            .arg("frag_keyframe+empty_moov") // enable streaming output
            .arg("pipe:1"); // output to stdout
        crate::Transcoder::new(cmd, reader, writer).map(Self)
    }

    pub fn abort(&self) {
        self.0.abort();
    }

    pub async fn wait(self) {
        self.0.wait().await;
    }
}

#[cfg(test)]
mod tests {
    use any_storage::{Store, StoreFile, StoreMetadata, WriteOptions};

    // requires to download https://download.blender.org/peach/bigbuckbunny_movies/BigBuckBunny_640x360.m4v
    // and store it in the .tmp directory
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
        let transcoder = super::Transcoder::new(source_reader, target_writer)
            .await
            .unwrap();
        transcoder.wait().await;
        //
        let meta = target_file.metadata().await.unwrap();
        assert_ne!(meta.size(), 0);
    }
}
