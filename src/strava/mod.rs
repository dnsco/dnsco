use reqwest;
use reqwest::header::AUTHORIZATION;

mod oauth;

pub use oauth::ClientConfig as OauthConfig;

pub fn authenticate(access_token: Option<String>, oauth_config: oauth::ClientConfig) -> StravaApi {
    let oauth_token = oauth::OauthToken(
        access_token.unwrap_or_else(|| oauth::oauth_dance(oauth_config).unwrap().access_token),
    );
    StravaApi { oauth_token }
}

#[derive(Debug, Clone)]
pub struct StravaApi {
    oauth_token: oauth::OauthToken,
}

impl StravaApi {
    pub fn activities(&self) -> Result<serde_json::Value, ()> {
        reqwest::Client::new()
            .get("https://www.strava.com/api/v3/athlete/activities")
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth_token.0))
            .send()
            .map_err(|_| ())?
            .json()
            .map_err(|_| ())
    }
}
