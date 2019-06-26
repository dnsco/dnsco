use askama::Template;

use dnsco_data::models::{activities, oauth_tokens};
use dnsco_data::{Database, RequestContext};
use strava;

use crate::app::SiteUrls;
use crate::{templates, AppError};

pub struct Service {
    db: Database,
    oauth_config: strava::OauthConfig,
    pub urls: SiteUrls,
}

impl Service {
    pub fn new(db: Database, oauth_config: strava::OauthConfig, urls: SiteUrls) -> Self {
        Self {
            db,
            oauth_config,
            urls,
        }
    }

    pub fn hello_world(&self) -> impl Template + '_ {
        templates::home::Index::new(&self.urls)
    }

    pub fn activities(&self) -> Result<templates::activities::List, AppError> {
        let context = RequestContext::new(&self.db, &self.oauth_config);
        let activities = context.activities_repo().all()?;
        Ok(templates::activities::List::new(activities, &self.urls))
    }

    pub fn update_activities(&self) -> Result<(), AppError> {
        let context = RequestContext::new(&self.db, &self.oauth_config);
        activities::commands::update_from_strava(context).map_err(AppError::from)
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
