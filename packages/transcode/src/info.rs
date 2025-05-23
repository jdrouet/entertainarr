use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};
use std::process::Stdio;
use std::str::FromStr;
use std::time::Duration;

use tokio::io::AsyncRead;

fn count_indent(input: &str) -> usize {
    input.chars().take_while(|c| *c == ' ').count()
}

fn parse_duration(input: &str) -> Option<Duration> {
    let (input, ms) = input.split_once('.').unwrap_or((input, "0"));
    let rest = ms.parse::<f64>().ok()?;
    let rest = rest * 0.1_f64.powi(ms.len() as i32);
    let mut seconds: u64 = 0;
    for cell in input.split(':') {
        seconds = seconds * 60 + cell.parse::<u64>().ok()?;
    }
    let seconds = seconds as f64 + rest;
    Some(Duration::from_secs_f64(seconds))
}

fn parse_metadata(indent: usize, lines: &mut Vec<&str>) -> HashMap<String, String> {
    let mut res = HashMap::default();
    while let Some(line) = lines.pop_if(|line| count_indent(line) == indent) {
        if let Some((name, value)) = line.split_once(':') {
            res.insert(name.trim().to_string(), value.trim().to_string());
        }
    }
    res
}

#[derive(Debug, Default)]
pub struct MediaInfo {
    pub metadata: HashMap<String, String>,
    pub duration: Option<Duration>,
}

impl MediaInfo {
    fn parse_duration(&mut self, line: &str) -> Result<()> {
        for (name, value) in line
            .split(',')
            .filter_map(|cell| cell.split_once(':'))
            .map(|(name, value)| (name.trim(), value.trim()))
        {
            if name == "Duration" {
                self.duration = parse_duration(value);
            }
        }
        Ok(())
    }
}

impl FromStr for MediaInfo {
    type Err = Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let mut res = Self::default();
        //
        let mut lines = input.split('\n').collect::<Vec<_>>();
        lines.reverse();
        // remove all the lines before `Input #0`
        while lines.pop_if(|line| !line.starts_with("Input #")).is_some() {}
        let _input = lines
            .pop()
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "unable to find `Input` line"))?;
        while let Some(next) = lines.pop_if(|line| count_indent(line) == 2) {
            let next = next.trim();
            if next.starts_with("Metadata:") {
                res.metadata = parse_metadata(4, &mut lines);
            } else if next.starts_with("Duration:") {
                res.parse_duration(next)?;
            }
        }

        Ok(res)
    }
}

impl MediaInfo {
    pub async fn from_reader(mut reader: impl AsyncRead + Send + Unpin + 'static) -> Result<Self> {
        let mut child = tokio::process::Command::new("ffprobe")
            .arg("-i")
            .arg("pipe:0")
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "unable to get stdin"))?;
        let copy_task = tokio::spawn(async move { tokio::io::copy(&mut reader, &mut stdin).await });
        let output = child.wait_with_output().await?;
        copy_task.abort();
        let _ = copy_task.await;
        let stderr = String::from_utf8_lossy(&output.stderr);
        MediaInfo::from_str(stderr.as_ref()).map_err(Error::other)
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Duration};

    use any_storage::{Store, StoreFile};

    const BIGBUCKBUNNY_STDOUT: &str = include_str!("../assets/bigbuckbunny-info-stdout.txt");

    #[tokio::test]
    async fn mediainfo_bigbuckbunny() {
        let storage = any_storage::local::LocalStore::new(".tmp");
        let file = storage.get_file("BigBuckBunny_640x360.m4v").await.unwrap();
        let reader = file.read(..).await.unwrap();
        //
        let _info = super::MediaInfo::from_reader(reader).await.unwrap();
    }

    #[test]
    fn parse_bigbuckbunny() {
        let res = super::MediaInfo::from_str(BIGBUCKBUNNY_STDOUT).unwrap();
        assert_eq!(res.metadata.get("major_brand").unwrap(), "M4V");
        assert_eq!(res.duration.unwrap().as_millis(), 596460);
    }

    #[test]
    fn parse_duration() {
        assert_eq!(
            super::parse_duration("00:00:13.00").unwrap(),
            Duration::new(13, 0)
        );
        assert_eq!(
            super::parse_duration("00:01:13.1").unwrap(),
            Duration::new(73, 100_000_000)
        );
    }
}
