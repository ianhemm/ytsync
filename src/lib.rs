pub mod request;
pub mod config;

use serde::{Deserialize, Serialize};

/**
 * The user facing struct that represents a video
 */
#[derive(Debug)]
pub struct Video {
    url: String,
    title: Option<String>,
    description: Option<String>,
    author: Option<String>,
}

impl Video {
    pub fn new(url: &str) -> Video {
        Video {
            url: url.to_string(),
            title: None,
            description: None,
            author: None,
        }
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }
    pub fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }
    pub fn set_author(&mut self, author: &str) {
        self.author = Some(author.to_string());
    }

    pub fn url(&self) -> &String {
        &self.url
    }
    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn author(&self) -> Option<&String> {
        self.author.as_ref()
    }
}

/*
 * Deserialization models
 */
/*
 * Playlist Deserialization
 */

/**
 * Instead of using the struct directly,
 * this is just going to be the model
 * to deserialize the playlist into a more usable form
 *
 * This is because of how youtube separates the data
 * into pages and a deeply nested structure.
pub  */
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct YoutubePlaylistPage {
    pub items: Vec<YoutubePlaylistItem>,
    pub next_page_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct YoutubePlaylistItem {
    pub snippet: YoutubeContentDescription,
}

impl YoutubePlaylistItem {
    pub fn to_video(&self) -> Video {
        const YOUTUBE_VIDEO_URL: &str = "https://youtube.com/watch?v=";
        let mut video = Video::new(&format!(
            "{}{}",
            YOUTUBE_VIDEO_URL, &self.snippet.resource_id.video_id
        ));

        video.set_description(&self.snippet.description);
        video.set_title(&self.snippet.title);
        if let Some(ref author) = self.snippet.video_owner_channel_title {
            video.set_author(author);
        }

        video
    }
}

/**
 * Idk why youtube API separates the video link like this
 * But ContentDescription holds pretty much every part of the video
 * Except for the downloadable link
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct YoutubeContentDescription {
    pub title: String,
    pub description: String,
    pub video_owner_channel_title: Option<String>,
    // TODO: Implement chrono datetime for finer control over playlist downloads
    // published_at: String,
    pub resource_id: YoutubeResourceId,
}

/**
 * what we use to construct the end result youtube link
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct YoutubeResourceId {
    pub video_id: String,
    kind: String,
}

/**
 * Subscription Deserialization
 */
#[derive(Serialize, Deserialize, Debug)]
struct Subscriptions {}
