use serde::{Deserialize, Serialize};

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
