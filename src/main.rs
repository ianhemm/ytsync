use reqwest::{self, Client};
use std::error::Error;
use tracing::info;

use ytsync::{config::Config, Video};
use youtube::{
    playlist::YoutubePlaylistPage,
    RequestBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /*
     * Initialization
     */

    let config = match Config::build() {
        Ok(x) => x,
        Err(e) => panic!("Error in config creation: {}",e),
    };

    // Logging system
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // request client
    let client = Client::builder().build()?;

    // Get list of playlists from a file
    const PLAYLIST_ID: &str = "PLbALbm1g5VzAqShkgKwo0NIVkwV9bZE8t"; // FIXME: test case that will represent the playlist we are wanting to pull videos from


	/*
     * Processing
     */
    let mut playlist: Vec<Video> = Vec::new();
    info!("Making a request to the first page of: {}", PLAYLIST_ID);
    let playlistitems_request = client
        .get(
            RequestBuilder::playlist_items(config.youtube_api())
                .max_items(50)
                .playlist_id(PLAYLIST_ID)
                .build())
        .header("accept", "application/json")
        .send()
        .await?;

    let response = playlistitems_request.text().await?;

    let mut page: YoutubePlaylistPage =
        serde_json::from_str(&response).expect("The data could not deserialize.");

    let mut links_page: Vec<Video> = page
        .items
        .into_iter()
        .map(|x| x.into())
        .collect();
    playlist.append(&mut links_page);

    while let Some(ref page_token) = page.next_page_token {
        info!("Making a request to the playlist: {}", PLAYLIST_ID);
        info!("Page: {}", page_token);
        // theres always going to be at least one request
        let request = client
            .get(
                RequestBuilder::playlist_items(config.youtube_api())
                    .max_items(50)
                    .playlist_id(PLAYLIST_ID)
                    .page_id(page_token)
                    .build(),
            )
            .header("accept", "application/json")
            .send()
            .await;

        let response = request?.text().await?;

        page = serde_json::from_str(&response).expect("The data could not deserialize.");
        let mut links_page: Vec<Video> = page
            .items
            .into_iter()
            .map(|x| x.into())
            .collect();
        playlist.append(&mut links_page);
    }

	/*
     * Output
     */
    println!("{:#?}", &playlist);

    Ok(())
}
