use crate::strava_api::StravaApi;
use crate::{domains, Database, DbConnection};
use domains::{activities::Repo as ActivitiesRepo, oauth_tokens::Repo as OauthRepo};

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

    pub fn strava_api(&self) -> StravaApi {
        StravaApi::new(self.oauth_config, self.tokens_repo())
    }
}
