use askama::Template;
use std::sync::{Arc, Mutex, MutexGuard};

use dnsco_data::{repos, Database, EventsRepo, StravaApi};
use repos::activities_repo;
use strava;

use crate::{config, templates};
use templates::{activities_template, index_template};

pub struct Webserver {
    db: Arc<Database>,
    events_repo: EventsRepo,
    strava_api: Arc<Mutex<StravaApi>>,
    pub urls: config::SiteUrls,
}

impl Webserver {
    pub fn new(
        db: Arc<Database>,
        strava_api: Arc<Mutex<StravaApi>>,
        urls: config::SiteUrls,
    ) -> Self {
        Self {
            db,
            events_repo: EventsRepo {},
            strava_api,
            urls,
        }
    }

    pub fn hello_world(&self) -> impl Template + '_ {
        let events = self.events_repo.events();
        index_template::new(events, &self.urls)
    }

    pub fn activities(&self) -> Result<String, ()> {
        let connection = self.db.get_connection();

        let repo = activities_repo::Repo {
            connection: &connection,
        };

        let string = repo
            .all()
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        Ok(string)
    }

    pub fn update_activities(&self) -> Result<impl Template, strava::Error> {
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
