use failure::Fail;
use reqwest;
use reqwest::header::AUTHORIZATION;

use crate::models::Activity;
use crate::oauth::OauthToken;
use crate::Error;

const ACTIVITIES_URL: &str = "https://www.strava.com/api/v3/athlete/activities";

#[derive(Debug)]
pub struct Api {
    oauth_token: OauthToken,
}

impl Api {
    pub fn new(oauth_token: OauthToken) -> Self {
        Self { oauth_token }
    }

    pub fn activities(&self) -> Result<Vec<Activity>, Error> {
        let mut response = self.activities_response()?;

        if response.status().is_success() {
            if let Ok(activities) = response.json() {
                return Ok(activities);
            }
        }

        Err(Api::parse_error(&mut response))
    }

    fn parse_error(response: &mut reqwest::Response) -> Error {
        match response.json() {
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
