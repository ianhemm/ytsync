pub mod playlist;

use std::borrow::Borrow;

#[derive(Debug)]
pub enum ClientError {
    NotAuthenticated,
}

const YT_API_URL: &str = "https://youtube.googleapis.com/youtube/v3";
/**
 * The api used to request and send data from and to youtube.
 * 
 * The crate uses the builder model in order to set and send
 * values to the api. 
 *
 */
pub struct Youtube<T> {
    token: T,
}

pub struct NoAuth;
pub struct ApiAuth(String);
pub struct OAuth(String);

pub trait Authorized {
    fn token(&self) -> &str;
    fn param_key(&self) -> &str;
}

impl<T:Borrow<ApiAuth>> Authorized for T{
    fn token(&self) -> &str {
		&self.borrow().0
    }

    fn param_key(&self) -> &str {
		"?key="
    }
}

impl Authorized for OAuth{
    fn token(&self) -> &str {
		&self.0
    }

	fn param_key(&self) -> &str {
    "?access_token="
    }
}

impl Youtube<NoAuth> {
    pub fn new() -> Youtube<NoAuth> {
        Youtube { token: NoAuth }
    }
}

impl Default for Youtube<NoAuth> {
    fn default() -> Self {
        Self::new()
    }
}

impl Youtube<NoAuth> {
    /**
     * Uses the simplified interface.
     * Any function that requires OAuth can not be used when the API key
     * is the authentication method.
     *
     * Most get requests are accessible, with the exception of those
     * that require user data.
     */
    pub fn with_api(&mut self, token: &str)
        -> Result<Youtube<ApiAuth>, ClientError> {
		Ok(Youtube { token: ApiAuth(token.into()) })
    }

    /**
     * Uses the full Youtube API interface
     *
     * Will return an interface that allows all youtube API requests to be sent
     * if successful.
     *
     * Note that, in order to use OAuth in a public setting, your program
     * needs to be reviewed by Google.
     * If you wish to get user data from youtube without OAuth,
     * its still possible.
     * However, the data being requested must be made public
     * on the users end, even if only temporarily.
     */
    pub fn with_oauth(&mut self, token: &str)
        -> Result<Youtube<OAuth>, ClientError> {
            Ok(Youtube { token: OAuth(token.into()) })
    }
}

/** Because OAuth connections can get data that Api connections can,
 * but not the other way around. We need to allow both to have access
 * without needing the OAuth state to have an API key as well.
 *
 * Note that given some commands also have parameters that require oauth
 * to work, we the request builder also require the same type
 * restrictions.
 */
impl<'a, T:Authorized> Youtube<T>{
    pub fn playlist_items(&'a self) -> YoutubePlaylistItemRequest<&'a T> {
        YoutubePlaylistItemRequest::new(&self.token)
    }
}

/**
 * Any command required by OAuth
 */
impl Youtube<OAuth> {}


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

impl<'a, T:Authorized> YoutubePlaylistItemRequest<T> {
    pub fn new(auth:&'a T) -> YoutubePlaylistItemRequest<&'a T> {
        YoutubePlaylistItemRequest {
            part: String::from("snippet"),
            playlist_id: None,
            max_items: None,
            page_id: None,
            auth,
        }
    }
}

impl<'a, T:Authorized> YoutubePlaylistItemRequest<T> {

    pub fn max_items(mut self, max_items: u32)
        -> YoutubePlaylistItemRequest<T> {
        self.max_items = Some(max_items);
        self
    }

    pub fn playlist_id(mut self, playlist_id: &str) -> YoutubePlaylistItemRequest<T> {
        self.playlist_id = Some(playlist_id.to_string());
        self
    }

    pub fn page_id(mut self, page_id: &str)
        -> YoutubePlaylistItemRequest<T>{
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
                &self.auth.token());

        request.to_string()
    }
}
