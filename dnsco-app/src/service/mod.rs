pub mod activities;
pub mod home;

use askama::Template;
use std::sync::{Arc, Mutex, MutexGuard};

use dnsco_data::{repos, Database, DbConnection, EventsRepo, StravaApi};
use repos::activities_repo;
use strava;

use crate::app::{AppError, SiteUrls};

pub struct Service {
    db: Arc<Database>,
    events_repo: EventsRepo,
    strava_api: Arc<Mutex<StravaApi>>,
    pub urls: SiteUrls,
}

impl Service {
    pub fn new(db: Arc<Database>, strava_api: Arc<Mutex<StravaApi>>, urls: SiteUrls) -> Self {
        Self {
            db,
            events_repo: EventsRepo {},
            strava_api,
            urls,
        }
    }

    pub fn hello_world(&self) -> impl Template + '_ {
        let events = self.events_repo.events();
        home::IndexTemplate::new(events, &self.urls)
    }

    pub fn activities(&self) -> Result<activities::ListTemplate, AppError> {
        let connection = self.db.get_connection();
        let activities = self.activities_repo(&connection).all();

        Ok(activities::ListTemplate::new(
            activities,
            self.urls.update_activities(),
        ))
    }

    pub fn update_activities(&self) -> Result<(), strava::Error> {
        let connection = self.db.get_connection();
        let strava_activities = self.get_strava_api().api()?.activities()?;

        self.activities_repo(&connection)
            .batch_upsert_from_strava(strava_activities);

        Ok(())
    }

    pub fn update_oauth_token(
        &self,
        oauth_resp: &strava::oauth::RedirectQuery,
    ) -> Result<strava::oauth::AccessTokenResponse, strava::Error> {
        let mut strava = self.get_strava_api();
        let resp = strava.parsed_oauth_response(&oauth_resp)?;
        strava.set_tokens(&resp);
        Ok(resp)
    }

    fn activities_repo<'a>(&self, connection: &'a DbConnection) -> activities_repo::Repo<'a> {
        activities_repo::Repo { connection }
    }

    fn get_strava_api(&self) -> MutexGuard<StravaApi> {
        self.strava_api.lock().unwrap()
    }
}
