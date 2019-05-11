use askama::Template;
use std::sync::{Arc, Mutex, MutexGuard};

use dnsco_data::{EventsRepo, StravaApi};
use strava;

use crate::{config, templates};
use templates::{activities_template, index_template};

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

    pub fn hello_world(&self) -> impl Template + '_ {
        let events = self.events_repo.events();
        index_template::new(events, &self.urls)
    }

    pub fn activities(&self) -> Result<impl Template, strava::Error> {
        let strava = self.get_strava_api().api()?.activities()?;
        let template = activities_template::new(strava);
        Ok(template)
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
