pub mod youtube {
    const YT_API_URL: &str = "https://youtube.googleapis.com/youtube/v3";

    /**
     * Youtube Request Models
     **/
    // builds a youtube request link to use in reqwest
    pub struct RequestBuilder;

    impl RequestBuilder {
        pub fn playlist_items(key: &str) -> YoutubePlaylistItemRequest {
            YoutubePlaylistItemRequest::new(key)
        }
    }

    pub struct YoutubePlaylistItemRequest {
        key: String,
        part: String,
        playlist_id: Option<String>,
        max_items: Option<u32>,
        page_id: Option<String>,
    }

    impl YoutubePlaylistItemRequest {
        pub fn new(key: &str) -> YoutubePlaylistItemRequest {
            YoutubePlaylistItemRequest {
                key: key.to_string(),
                part: String::from("snippet"),
                playlist_id: None,
                max_items: None,
                page_id: None,
            }
        }

        pub fn max_items(mut self, max_items: u32) -> YoutubePlaylistItemRequest {
            self.max_items = Some(max_items);
            self
        }

        pub fn playlist_id(mut self, playlist_id: &str) -> YoutubePlaylistItemRequest {
            self.playlist_id = Some(playlist_id.to_string());
            self
        }

        pub fn page_id(mut self, page_id: &str) -> YoutubePlaylistItemRequest {
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

            request = format!("{}&key={}", request, self.key);

            request.to_string()
        }
    }
}
