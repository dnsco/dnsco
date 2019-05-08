use crate::{config, templates};
use std::sync::{Arc, Mutex, MutexGuard};

use dnsco_data::{EventsRepo, StravaApi};
use strava;

use templates::IndexTemplate;

pub struct Webserver {
    events_repo: EventsRepo,
    strava_api: Arc<Mutex<StravaApi>>,
    urls: config::SiteUrls,
}

impl Webserver {
    pub fn new(strava_api: Arc<Mutex<StravaApi>>, urls: config::SiteUrls) -> Self {
        let events_repo = EventsRepo {};
        Self {
            events_repo,
            strava_api,
            urls,
        }
    }

    pub fn hello_world(&self) -> IndexTemplate {
        let events = self.events_repo.events();
        IndexTemplate {
            events,
            urls: &self.urls,
        }
    }

    pub fn activities(&self) -> Result<String, strava::Error> {
        match self.get_strava_api().api()?.activities() {
            Ok(activities) => serde_json::to_string(&activities).map_err(strava::Error::Parse),
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
