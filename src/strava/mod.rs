use reqwest;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::fmt;
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

const ACTIVITIES_URL: &'static str = "https://www.strava.com/api/v3/athlete/activities";

impl Api {
    pub fn activities(&self) -> Result<Vec<Activity>, StravaError> {
        let mut response = self.activities_response()?;

        if response.status().is_success() {
            if let Ok(activities) = response.json() {
                return Ok(activities);
            }
        }

        return Err(Api::parse_error(&mut response));
    }

    fn parse_error(mut response: &mut reqwest::Response) -> StravaError {
        StravaError::ApiError(response.json().unwrap_or_else(|_| {
            let message = match response.text() {
                Ok(resp) => format!("Strava Api Returned: {}", resp),
                Err(_) => "Failed to read http response".to_owned(),
            };

            ErrorResponse {
                errors: Vec::new(),
                message,
            }
        }))
    }

    fn activities_response(&self) -> Result<reqwest::Response, StravaError> {
        reqwest::Client::new()
            .get(ACTIVITIES_URL)
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth_token.0))
            .send()
            .map_err(|e| StravaError::NetworkError(Box::new(e)))
    }
}

#[derive(Debug)]
pub enum StravaError {
    ApiError(ErrorResponse),
    NetworkError(Box<std::error::Error>),
}

impl fmt::Display for StravaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    errors: Vec<serde_json::Value>,
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
