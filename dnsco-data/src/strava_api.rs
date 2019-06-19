use crate::domains::oauth_tokens::Repo;
use strava::oauth;

pub struct StravaApi {
    oauth_config: oauth::ClientConfig,
}

impl StravaApi {
    pub fn new(oauth_config: oauth::ClientConfig) -> Self {
        Self { oauth_config }
    }

    pub fn api(&self, token_repo: Repo) -> Result<strava::Api, strava::Error> {
        match token_repo.get().ok() {
            Some(token) => Ok(strava::Api::new(oauth::OauthToken(token.token))),
            None => Err(strava::Error::NoOauthToken(oauth::get_authorization_url(
                &self.oauth_config,
            ))),
        }
    }

    pub fn parsed_oauth_response(
        &self,
        oauth_resp: &oauth::RedirectQuery,
    ) -> Result<oauth::AccessTokenResponse, strava::Error> {
        oauth::redirect_callback(&oauth_resp, &self.oauth_config)
    }
}
