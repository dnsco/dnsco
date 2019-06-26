use crate::models::oauth_tokens::Repo;
use strava::oauth;

pub struct StravaApi<'a> {
    oauth_config: &'a oauth::ClientConfig,
    token_repo: Repo<'a>,
}

impl<'a> StravaApi<'a> {
    pub fn new(oauth_config: &'a oauth::ClientConfig, repo: Repo<'a>) -> Self {
        Self {
            oauth_config,
            token_repo: repo,
        }
    }

    pub fn api(&self) -> Result<strava::Api, strava::Error> {
        match self.token_repo.get().ok() {
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
        oauth::redirect_callback(&oauth_resp, self.oauth_config)
    }
}
