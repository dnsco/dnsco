pub mod activities;
pub mod home;

use askama::Template;

use dnsco_data::{Database, EventsRepo, RequestContext};

use strava;

use crate::app::SiteUrls;
use crate::AppError;
use dnsco_data::domains::oauth_tokens;

pub struct Service {
    db: Database,
    events_repo: EventsRepo,
    oauth_config: strava::OauthConfig,
    pub urls: SiteUrls,
}

impl Service {
    pub fn new(db: Database, oauth_config: strava::OauthConfig, urls: SiteUrls) -> Self {
        Self {
            db,
            events_repo: EventsRepo {},
            oauth_config,
            urls,
        }
    }

    pub fn hello_world(&self) -> impl Template + '_ {
        let events = self.events_repo.events();
        home::IndexTemplate::new(events, &self.urls)
    }

    pub fn activities(&self) -> Result<activities::ListTemplate, AppError> {
        let context = RequestContext::new(&self.db, &self.oauth_config);
        let activities = context.activities_repo().all();

        Ok(activities::ListTemplate::new(
            activities,
            self.urls.update_activities(),
        ))
    }

    pub fn update_activities(&self) -> Result<(), strava::Error> {
        let context = RequestContext::new(&self.db, &self.oauth_config);

        let strava_api = context.strava_api().api()?;

        context
            .activities_repo()
            .batch_upsert_from_strava(strava_api.activities()?);

        Ok(())
    }

    pub fn update_oauth_token(
        &self,
        oauth_resp: &strava::oauth::RedirectQuery,
    ) -> Result<(), AppError> {
        let context = RequestContext::new(&self.db, &self.oauth_config);
        oauth_tokens::commands::update_from_strava(&context, oauth_resp)?;
        Ok(())
    }
}
