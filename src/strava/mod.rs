use url::Url;

use failure::Fail;
use oauth::OauthToken;

mod api;
mod errors;
mod models;
pub mod oauth;

pub use api::Api;
pub use errors::Error;

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

    pub fn set_tokens(&mut self, resp: &oauth::AccessTokenResponse) {
        self.oauth_token = Some(OauthToken(resp.oauth_token()));
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

    pub fn parsed_oauth_response(
        &self,
        oauth_resp: &oauth::RedirectQuery,
    ) -> Result<oauth::AccessTokenResponse, impl Fail> {
        oauth::redirect_callback(&oauth_resp, &self.oauth_config)
    }
}
