use failure::Fail;
use reqwest;
use reqwest::header::AUTHORIZATION;

use crate::models::activity;
use crate::oauth::OauthToken;
use crate::{models, Error};

const ACTIVITIES_URL: &str = "https://www.strava.com/api/v3/athlete/activities";

#[derive(Debug)]
pub struct Api {
    oauth_token: OauthToken,
}

impl Api {
    pub fn new(oauth_token: OauthToken) -> Self {
        Self { oauth_token }
    }

    pub fn activities(&self) -> Result<Vec<activity::Summary>, Error> {
        let mut response = self.activities_response()?;

        if response.status().is_success() {
            if let Ok(activities) = response.json() {
                return Ok(activities);
            }
        }

        Err(response.into())
    }

    fn activities_response(&self) -> Result<reqwest::Response, Error> {
        reqwest::Client::new()
            .get(ACTIVITIES_URL)
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth_token.0))
            .send()
            .map_err(|e| {
                Error::NetworkError(Box::new(e.context("Failed to fetch Strava Activities")))
            })
    }
}

impl From<reqwest::Response> for Error {
    fn from(mut response: reqwest::Response) -> Self {
        match response.json::<models::ErrorResponse>() {
            Ok(api_error) => Error::ApiError(api_error),
            Err(json_error) => {
                let context = match response.text() {
                    Ok(body) => format!("Failed to parse strava api response: {}", body),
                    Err(e) => format!("Can't read strava api response body: {}", e),
                };

                Error::NetworkError(Box::new(json_error.context(context)))
            }
        }
    }
}
