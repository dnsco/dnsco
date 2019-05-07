use crate::{config, service, strava, templates};
use std::sync::{Arc, Mutex, MutexGuard};

use service::models::{Event, Race};
use service::StravaApi;
use templates::IndexTemplate;

pub struct Webserver {
    strava_api: Arc<Mutex<StravaApi>>,
    events: Vec<Event>,
    urls: config::SiteUrls,
}

impl Webserver {
    pub fn new(strava_api: Arc<Mutex<StravaApi>>, urls: config::SiteUrls) -> Self {
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

    pub fn activities(&self) -> Result<String, strava::Error> {
        match self.get_strava_api().api()?.activities() {
            Ok(activities) => {
                serde_json::to_string(&activities).map_err(|e| strava::Error::Parse(e))
            }
            Err(error) => Err(error),
        }
    }

    fn get_strava_api(&self) -> MutexGuard<StravaApi> {
        self.strava_api.lock().unwrap()
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
