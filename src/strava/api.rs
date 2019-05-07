use crate::strava::errors::Error as StravaError;
use crate::strava::models::Activity;
use crate::strava::oauth::OauthToken;

use failure::Fail;
use reqwest;
use reqwest::header::AUTHORIZATION;

const ACTIVITIES_URL: &str = "https://www.strava.com/api/v3/athlete/activities";

#[derive(Debug)]
pub struct Api {
    oauth_token: OauthToken,
}

impl Api {
    pub fn new(oauth_token: OauthToken) -> Self {
        Self { oauth_token }
    }

    pub fn activities(&self) -> Result<Vec<Activity>, StravaError> {
        let mut response = self.activities_response()?;

        if response.status().is_success() {
            if let Ok(activities) = response.json() {
                return Ok(activities);
            }
        }

        Err(Api::parse_error(&mut response))
    }

    fn parse_error(response: &mut reqwest::Response) -> StravaError {
        match response.json() {
            Ok(api_error) => StravaError::ApiError(api_error),
            Err(json_error) => {
                let context = match response.text() {
                    Ok(body) => format!("Failed to parse strava api response: {}", body),
                    Err(e) => format!("Can't read strava api response body: {}", e),
                };

                StravaError::NetworkError(Box::new(json_error.context(context)))
            }
        }
    }

    fn activities_response(&self) -> Result<reqwest::Response, StravaError> {
        reqwest::Client::new()
            .get(ACTIVITIES_URL)
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth_token.0))
            .send()
            .map_err(|e| {
                StravaError::NetworkError(Box::new(e.context("Failed to fetch Strava Activities")))
            })
    }
}
