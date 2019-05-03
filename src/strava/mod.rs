use reqwest;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

mod oauth;

pub use oauth::ClientConfig as OauthConfig;
use std::io::Read;

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
        let mut response = self.activities_response()?;
        if response.status().is_success() {
            return response.json().map_err(|_| "Failed to Parse Json");
        } else {
            dbg!(&response);
            dbg!(response
                .text()
                .map_err(|_| "Failed to read http response body")?);
            return Err("Unsusessful response");
        }
    }

    fn activities_response(&self) -> Result<reqwest::Response, &'static str> {
        reqwest::Client::new()
            .get("https://www.strava.com/api/v3/athlete/activities")
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth_token.0))
            .send()
            .map_err(|_| "Strava Network Error")
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    errors: serde_json::Value,
    message: String,
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
