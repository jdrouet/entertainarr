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
#[allow(unused, reason = "no methods to list episodes yet")]
pub struct PodcastEpisode {
    pub id: u64,
    pub podcast_id: u64,
    pub guid: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub duration: Option<Duration>,
    pub file_url: String,
    pub file_size: Option<u64>,
    pub file_type: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct PodcastInput {
    pub feed_url: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub language: Option<String>,
    pub website: Option<String>,
    pub episodes: Vec<PodcastEpisodeInput>,
}

#[derive(Debug)]
pub struct PodcastEpisodeInput {
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
