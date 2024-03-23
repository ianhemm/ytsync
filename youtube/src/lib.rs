mod auth;
pub mod playlist;
use crate::auth::{ApiAuth, Authorized, NoAuth, OAuth};
use crate::playlist::YoutubePlaylistItemRequest;

// Constants
const YT_API_URL: &str = "https://youtube.googleapis.com/youtube/v3";

#[derive(Debug)]
pub enum ClientError {
    NotAuthenticated,
    QuotaExceeded,
}

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
    pub fn with_api(&mut self, token: &str) -> Result<Youtube<ApiAuth>, ClientError> {
        Ok(Youtube {
            token: ApiAuth(token.into()),
        })
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
    pub fn with_oauth(&mut self, token: &str) -> Result<Youtube<OAuth>, ClientError> {
        Ok(Youtube {
            token: OAuth(token.into()),
        })
    }
}

/** Because OAuth connections can get data that Api connections can,
 * but not the other way around. We need to allow both to have access
 * without needing the OAuth state to have an API key as well.
 *
 * Note that given some commands also have parameters that require oauth
 * to work, the request builder also requires the same type
 * restrictions.
 */
impl<'a, T: Authorized> Youtube<T> {
    pub fn playlist_items(&'a self) -> YoutubePlaylistItemRequest<&'a T> {
        YoutubePlaylistItemRequest::new(&self.token)
    }
}

/**
 * Any command required by OAuth
 */
impl Youtube<OAuth> {}
