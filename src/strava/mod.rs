use url::Url;

use oauth::OauthToken;

pub use api::Api;
pub use oauth::{
    redirect_callback as oauth_redirect_callback, ClientConfig as OauthConfig,
    RedirectQuery as OauthRedirectQuery,
};

mod api;
mod errors;
mod models;
mod oauth;

#[derive(Debug)]
pub struct AuthedApi {
    oauth_token: Option<OauthToken>,
    oauth_config: oauth::ClientConfig,
}

impl AuthedApi {
    pub fn new(access_token: Option<String>, oauth_config: oauth::ClientConfig) -> Self {
        let oauth_token = access_token.map(|token| OauthToken(token));
        Self {
            oauth_config,
            oauth_token,
        }
    }

    pub fn api(&self) -> Result<Api, errors::Error> {
        match &self.oauth_token {
            Some(token) => Ok(Api::new(token.clone())),
            None => Err(errors::Error::BadOauthToken),
        }
    }

    pub fn auth_url(&self) -> Url {
        oauth::get_authorization_url(&self.oauth_config)
    }
}
