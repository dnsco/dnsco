use crate::strava_api::StravaApi;
use crate::{models, Database, DbConnection};
use models::{activities::Repo as ActivitiesRepo, oauth_tokens::Repo as OauthRepo};
use strava::oauth::StravaOauth;

pub struct RequestContext<'a> {
    connection: DbConnection,
    oauth_config: &'a strava::OauthConfig,
}

impl<'a> RequestContext<'a> {
    pub fn new(db: &Database, oauth_config: &'a strava::OauthConfig) -> Self {
        Self {
            connection: db.get_connection(),
            oauth_config,
        }
    }

    pub fn activities_repo(&self) -> ActivitiesRepo {
        ActivitiesRepo {
            connection: &self.connection,
        }
    }

    pub fn tokens_repo(&self) -> OauthRepo {
        OauthRepo {
            connection: &self.connection,
        }
    }

    pub fn strava_api(&self) -> StravaApi<StravaOauth<'a>> {
        StravaApi::new(self.oauth_config)
    }
}
