use crate::{strava, SiteUrls};

use serde::Serialize;
use std::sync::{Arc, Mutex};

use askama::Template;
use failure::Fail;
use url::Url;

pub struct Webserver {
    strava_api: Arc<Mutex<strava::AuthedApi>>,
    events: Vec<Event>,
    urls: SiteUrls,
}

#[derive(Serialize, Clone, Debug)]
struct Event {
    name: &'static str,
    time: &'static str,
    info: Race,
}

#[derive(Serialize, Clone, Debug)]
struct Race {
    distance: &'static str,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    events: Vec<Event>,
    urls: &'a SiteUrls,
}

impl Webserver {
    pub fn new(strava_api: Arc<Mutex<strava::AuthedApi>>, urls: SiteUrls) -> Self {
        Self {
            events: vec![
                Event {
                    name: "Marin Ultra Challenge",
                    time: "2019-03-09",
                    info: Race { distance: "25k " },
                },
                Event {
                    name: "Behind the Rocks",
                    time: "2019-03-23",
                    info: Race { distance: "30k" },
                },
                Event {
                    name: "Broken Arrow Skyrace",
                    time: "2019-06-23",
                    info: Race { distance: "26k " },
                },
            ],
            strava_api,
            urls,
        }
    }

    pub fn hello_world(&self) -> IndexTemplate {
        IndexTemplate {
            events: self.events.to_vec(),
            urls: &self.urls,
        }
    }

    pub fn activities(&self, api: &strava::Api) -> Result<String, String> {
        match api.activities() {
            Ok(activities) => serde_json::to_string(&activities).map_err(reserialization_failure),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn get_strava_api(&self) -> Result<strava::Api, strava::Error> {
        self.strava_api.lock().unwrap().api()
    }

    pub fn oauth_redirect_url(&self) -> Url {
        self.strava_api.lock().unwrap().auth_url()
    }

    pub fn update_oauth_token(
        &self,
        oauth_resp: &strava::oauth::RedirectQuery,
    ) -> Result<strava::oauth::AccessTokenResponse, strava::Error> {
        let mut strava = self.strava_api.lock().unwrap();
        let resp = strava.parsed_oauth_response(&oauth_resp)?;
        strava.set_tokens(&resp);
        Ok(resp)
    }
}

fn reserialization_failure(_: serde_json::Error) -> String {
    "Failed to Reserialize".to_owned()
}
