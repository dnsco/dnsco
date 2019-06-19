pub mod activities;
pub mod home;

use askama::Template;
use std::sync::Arc;

use dnsco_data::{ActivitiesRepo, Database, DbConnection, EventsRepo, OauthRepo, StravaApi};
use strava;

use crate::app::SiteUrls;
use crate::AppError;

pub struct Service {
    db: Arc<Database>,
    events_repo: EventsRepo,
    strava_api: Arc<StravaApi>,
    pub urls: SiteUrls,
}

impl Service {
    pub fn new(db: Arc<Database>, strava_api: Arc<StravaApi>, urls: SiteUrls) -> Self {
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
        let tokens = self.tokens_repo(&connection);
        let strava_api = self.strava_api.api(tokens)?;

        self.activities_repo(&connection)
            .batch_upsert_from_strava(strava_api.activities()?);

        Ok(())
    }

    pub fn update_oauth_token(
        &self,
        oauth_resp: &strava::oauth::RedirectQuery,
    ) -> Result<(), AppError> {
        let resp = self
            .strava_api
            .parsed_oauth_response(&oauth_resp)
            .map_err(AppError::StravaError)?;
        let connection = self.db.get_connection();
        self.tokens_repo(&connection)
            .upsert(&resp)
            .map_err(AppError::QueryError)?;

        Ok(())
    }

    fn activities_repo<'a>(&self, connection: &'a DbConnection) -> ActivitiesRepo<'a> {
        ActivitiesRepo { connection }
    }

    fn tokens_repo<'a>(&self, connection: &'a DbConnection) -> OauthRepo<'a> {
        OauthRepo { connection }
    }
}
