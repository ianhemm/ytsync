use crate::auth::Authorized;
use crate::YT_API_URL;
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

/**
 * Youtube Request Models
 **/
// builds a youtube request link to use in reqwest
pub struct YoutubePlaylistItemRequest<T> {
    part: String,
    playlist_id: Option<String>,
    max_items: Option<u32>,
    page_id: Option<String>,

    auth: T,
}

impl<'a, T: Authorized> YoutubePlaylistItemRequest<T> {
    pub fn new(auth: &'a T) -> YoutubePlaylistItemRequest<&'a T> {
        YoutubePlaylistItemRequest {
            part: String::from("snippet"),
            playlist_id: None,
            max_items: None,
            page_id: None,
            auth,
        }
    }

    pub fn max_items(mut self, max_items: u32) -> YoutubePlaylistItemRequest<T> {
        self.max_items = Some(max_items);
        self
    }

    pub fn playlist_id(mut self, playlist_id: &str) -> YoutubePlaylistItemRequest<T> {
        self.playlist_id = Some(playlist_id.to_string());
        self
    }

    pub fn page_id(mut self, page_id: &str) -> YoutubePlaylistItemRequest<T> {
        self.page_id = Some(page_id.to_string());
        self
    }

    pub fn build(self) -> String {
        let mut request = format!("{}/playlistItems?part={}", YT_API_URL, self.part);

        if let Some(id) = self.playlist_id {
            request = format!("{}&playlistId={}", request, id);
        }

        if let Some(max) = self.max_items {
            request = format!("{}&maxResults={}", request, max)
        }

        if let Some(page) = self.page_id {
            request = format!("{}&pageToken={}", request, page)
        }

        request = format!(
            "{}{}{}",
            request,
            &self.auth.param_key(),
            &self.auth.token()
        );

        request.to_string()
    }
}
