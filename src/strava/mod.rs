use reqwest;
use reqwest::header::AUTHORIZATION;

mod oauth;

pub use oauth::ClientConfig as OauthConfig;

pub fn authenticate(access_token: Option<String>, oauth_config: oauth::ClientConfig) -> Api {
    let oauth_token = oauth::OauthToken(
        access_token.unwrap_or_else(|| oauth::oauth_dance(oauth_config).unwrap().access_token),
    );
    Api { oauth_token }
}

#[derive(Debug)]
pub struct Api {
    oauth_token: oauth::OauthToken,
}

impl Api {
    pub fn activities(&self) -> Result<Vec<Activity>, &'static str> {
        reqwest::Client::new()
            .get("https://www.strava.com/api/v3/athlete/activities")
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth_token.0))
            .send()
            .map_err(|_| "Strava Network Error")?
            .json()
            .map_err(|_| "Failed to Parse Json")
    }
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    name: String,
    distance: f32,
    total_elevation_gain: f32,
    athlete: Athlete,
}

#[derive(Serialize, Deserialize)]
pub struct Athlete {
    id: usize,
}
