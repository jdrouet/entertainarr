use std::cell::LazyCell;

use entertainarr_storage::entry::FileInfo;

const TVSHOW_FILENAME_PARSER: LazyCell<TVShowFilenameParser> =
    LazyCell::new(TVShowFilenameParser::default);

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    Audio,
    Image,
    Video,
}

impl FileType {
    pub fn detect(name: &str) -> Option<Self> {
        let ext = name.rsplit_once('.').map(|(_, ext)| ext)?;
        let guess = mime_guess::from_ext(ext);
        for mime in guess.iter() {
            match mime.type_() {
                mime_guess::mime::AUDIO => {
                    return Some(Self::Audio);
                }
                mime_guess::mime::VIDEO => {
                    return Some(Self::Video);
                }
                mime_guess::mime::IMAGE => {
                    return Some(Self::Image);
                }
                _ => {}
            }
        }
        None
    }
}

#[derive(Debug)]
pub(super) struct AnalyseFile {
    pub source: String,
    pub path: String,
    pub file: FileInfo,
}

impl AnalyseFile {
    fn relative_path(&self) -> String {
        if self.path.is_empty() {
            self.file.name.clone()
        } else {
            format!("{}/{}", self.path, self.file.name)
        }
    }

    async fn analyze_tvshow(
        &self,
        ctx: &super::Context,
        filepath: String,
        file: TVShowFile,
    ) -> Result<(), super::Error> {
        tracing::debug!(
            message = "found tvshow",
            title = file.title,
            season = file.season,
            episode = file.episode
        );
        Ok(())
    }

    async fn analyse_video(&self, ctx: &super::Context) -> Result<(), super::Error> {
        let filepath = self.relative_path();
        if let Some(tvshow) = TVSHOW_FILENAME_PARSER.detect(&filepath) {
            self.analyze_tvshow(ctx, filepath, tvshow).await?;
        }
        Ok(())
    }

    #[tracing::instrument(name = "analyze_file", skip_all, fields(source = %self.source, path = %self.path, filename = %self.file.name))]
    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), super::Error> {
        let Some(filetype) = FileType::detect(&self.file.name) else {
            tracing::debug!("unable to detect type");
            return Ok(());
        };
        match filetype {
            FileType::Video => self.analyse_video(ctx).await,
            kind => {
                tracing::debug!("{kind:?} not yet implemented");
                Ok(())
            }
        }
    }
}

struct TVShowFilenameParser {
    regex: regex::Regex,
}

impl Default for TVShowFilenameParser {
    fn default() -> Self {
        Self {
            regex: regex::Regex::new(r#"(?:([^\/]+)(?:[_\-\s]+(\d{4}))|([^\/]+))[_\-\s]+(?:[sS](\d+)[eE](\d+))(?:.*)\.\w+$"#).unwrap(),
        }
    }
}

impl TVShowFilenameParser {
    fn detect(&self, filepath: &str) -> Option<TVShowFile> {
        let cap = self.regex.captures(filepath)?;
        let title = cap
            .get(1)
            .or_else(|| cap.get(3))
            .map(|v| v.as_str().to_lowercase())?;
        let title = title
            .replace("-", " ")
            .replace("_", " ")
            .split(" ")
            .filter(|item| !item.is_empty())
            .fold(String::default(), |mut acc, item| {
                if !acc.is_empty() {
                    acc.push(' ');
                }
                acc.push_str(item);
                acc
            });
        let year = cap.get(2).and_then(|v| v.as_str().parse::<u16>().ok());
        let season = cap.get(4).and_then(|v| v.as_str().parse::<u16>().ok())?;
        let episode = cap.get(5).and_then(|v| v.as_str().parse::<u16>().ok())?;

        Some(TVShowFile {
            title,
            year,
            season,
            episode,
        })
    }
}

struct TVShowFile {
    title: String,
    year: Option<u16>,
    season: u16,
    episode: u16,
}

#[cfg(test)]
mod tests {
    use super::TVShowFilenameParser;

    #[test_case::test_case("the-umbrella-academy_2019/S01/The_Umbrella_Academy_S01E05.mkv", "the umbrella academy", None, 1, 5; "name, season and episode")]
    #[test_case::test_case("the-simpsons_1989/S36/the-simpsons_1989_S36E07.mkv", "the simpsons", Some(1989), 36, 7; "with year in filename")]
    #[test_case::test_case("disenchantment_2018/S01/disenchantment_2018_S01E13.mkv", "disenchantment", Some(2018), 1, 13; "single word")]
    #[test_case::test_case("big-mouth_2017/S01/big-mouth_2017_S01E03.mkv", "big mouth", Some(2017), 1, 3; "different space schemes")]
    fn should_extract_tvshow_information(
        filepath: &str,
        title: &str,
        year: Option<u16>,
        season: u16,
        episode: u16,
    ) {
        let parser = TVShowFilenameParser::default();
        let detected = parser.detect(filepath).unwrap();
        assert_eq!(detected.title, title);
        assert_eq!(detected.year, year);
        assert_eq!(detected.season, season);
        assert_eq!(detected.episode, episode);
    }
}
