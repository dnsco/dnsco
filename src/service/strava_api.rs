use crate::strava;
use strava::oauth;
use strava::oauth::OauthToken;

pub struct StravaApi {
    oauth_token: Option<OauthToken>,
    oauth_config: oauth::ClientConfig,
}

impl StravaApi {
    pub fn new(access_token: Option<String>, oauth_config: oauth::ClientConfig) -> Self {
        let oauth_token = access_token.map(OauthToken);
        Self {
            oauth_config,
            oauth_token,
        }
    }

    pub fn set_tokens(&mut self, resp: &oauth::AccessTokenResponse) {
        self.oauth_token = Some(OauthToken(resp.oauth_token()));
    }

    pub fn api(&self) -> Result<strava::Api, strava::Error> {
        match &self.oauth_token {
            Some(token) => Ok(strava::Api::new(token.clone())),
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
