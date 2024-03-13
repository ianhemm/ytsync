pub mod request;
pub mod config;

use youtube::playlist::YoutubePlaylistItem;

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

impl From<YoutubePlaylistItem> for Video {
    fn from(value: YoutubePlaylistItem) -> Self {
        const YOUTUBE_VIDEO_URL: &str = "https://youtube.com/watch?v=";
        let mut video = Video::new(&format!(
            "{}{}",
            YOUTUBE_VIDEO_URL, &value.snippet.resource_id.video_id
        ));

        video.set_description(&value.snippet.description);
        video.set_title(&value.snippet.title);
        if let Some(ref author) = &value.snippet.video_owner_channel_title {
            video.set_author(author);
        }

        video
    }
}
