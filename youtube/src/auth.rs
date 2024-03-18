use std::borrow::Borrow;

/**
 * Type State for authorizing youtube requests with either Oauth
 * or basic key access depending on your needs
 *
 * The Youtube class is made in a way where you should not need to use
 * these structs directly.
 */
pub struct NoAuth;
pub struct ApiAuth(pub String);
pub struct OAuth(pub String);

pub trait Authorized {
    fn token(&self) -> &str;
    fn param_key(&self) -> &str;
}

impl<T: Borrow<ApiAuth>> Authorized for T {
    fn token(&self) -> &str {
        &self.borrow().0
    }

    fn param_key(&self) -> &str {
        "&key="
    }
}

impl Authorized for OAuth {
    fn token(&self) -> &str {
        &self.0
    }

    fn param_key(&self) -> &str {
        "&access_token="
    }
}
