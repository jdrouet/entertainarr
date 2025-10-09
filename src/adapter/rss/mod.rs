use std::time::Duration;

mod parser;
mod podcast;
mod tracing;

#[derive(Debug)]
pub struct Config;

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub fn build(self) -> anyhow::Result<RssClient> {
        let client = reqwest::Client::new();
        let client = reqwest_middleware::ClientBuilder::new(client)
            .with(tracing::TracingMiddleware::default())
            .build();
        Ok(RssClient { client })
    }
}

#[derive(Clone, Debug)]
pub struct RssClient {
    client: reqwest_middleware::ClientWithMiddleware,
}

#[derive(Debug, Default)]
struct Rss {
    channels: Vec<Channel>,
}

#[derive(Debug, Default)]
struct Channel {
    link: Option<String>,
    title: Option<String>,
    description: Option<String>,
    image_link: Option<String>,
    image_url: Option<String>,
    image_title: Option<String>,
    language: Option<String>,
    publication_date: Option<chrono::DateTime<chrono::Utc>>,
    last_build_date: Option<chrono::DateTime<chrono::Utc>>,
    managing_editor: Option<String>,
    web_master: Option<String>,
    // atom namespace
    atom_link_href: Option<String>,
    // itunes namespace
    itunes_author: Option<String>,
    itunes_category: Option<String>,
    itunes_explicit: Option<bool>,
    itunes_image_href: Option<String>,
    itunes_owner_email: Option<String>,
    itunes_owner_name: Option<String>,
    itunes_subtitle: Option<String>,
    itunes_summary: Option<String>,
    //
    items: Vec<ChannelItem>,
}

#[derive(Debug, Default)]
pub struct ChannelItem {
    title: Option<String>,
    description: Option<String>,
    link: Option<String>,
    guid: Option<String>,
    guid_perma_link: Option<bool>,
    publication_date: Option<chrono::DateTime<chrono::Utc>>,
    content_encoded: Option<String>,
    enclosure_url: Option<String>,
    enclosure_length: Option<u64>,
    enclosure_type: Option<String>,
    itunes_duration: Option<Duration>,
    itunes_summary: Option<String>,
}
