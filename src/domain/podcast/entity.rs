use std::time::Duration;

#[derive(Debug)]
pub struct Podcast {
    pub id: u64,
    pub feed_url: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub language: Option<String>,
    pub website: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct PodcastWithEpisodes {
    pub podcast: Podcast,
    pub episodes: Vec<PodcastEpisode>,
}

#[derive(Debug)]
pub struct PodcastEpisode {
    pub guid: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub duration: Option<Duration>,
    pub file_url: String,
    pub file_size: Option<u64>,
    pub file_type: Option<String>,
}
