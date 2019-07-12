use crate::models::oauth_tokens::OauthToken;
use chrono::Utc;
use strava::oauth;
use strava::oauth::{OauthClient, StravaOauth};

pub struct StravaApi<T>
where
    T: OauthClient,
{
    pub oauth_client: T,
}

impl<'a> StravaApi<StravaOauth<'a>> {
    pub fn new(oauth_config: &'a oauth::ClientConfig) -> StravaApi<StravaOauth<'a>> {
        let oauth_client = StravaOauth::new(oauth_config);
        Self { oauth_client }
    }
}

impl<T> StravaApi<T>
where
    T: OauthClient,
{
    pub fn api(&self, t: Option<OauthToken>) -> Result<strava::Api, strava::Error> {
        dbg!("making api");
        if let Some(db_token) = t {
            let token = if db_token.expires_at > Utc::now() {
                db_token.token
            } else {
                dbg!("refreshing");
                let resp = self.oauth_client.refresh_token(db_token.refresh);
                dbg!(&resp);
                resp?.access
            };

            return Ok(strava::Api::new(oauth::OauthToken(token)));
        }

        let redirect_url = self.oauth_client.get_authorization_url();
        Err(strava::Error::NoOauthToken(redirect_url))
    }

    pub fn get_token_from_code(
        &self,
        oauth_resp: &oauth::RedirectQuery,
    ) -> Result<oauth::AccessTokenResponse, strava::Error> {
        self.oauth_client.exchange_code(oauth_resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use strava::models::athlete::Summary;
    use strava::oauth::{AccessTokenResponse, RedirectQuery};
    use strava::Error as StravaError;
    use url::Url;

    #[test]
    fn test_api_works_with_valid_function() {
        let api: StravaApi<FakeOauth> = StravaApi::<FakeOauth>::new();
        let token = fake_oauth_token();
        assert!(api.api(Some(token)).is_ok());
    }

    #[test]
    fn api_refreshes_expired_token() {
        let mut api = StravaApi::<FakeOauth>::new();
        let mut expired = fake_oauth_token();
        expired.expires_at = Utc::now() - chrono::Duration::hours(1);

        api.oauth_client.should_succeed = false;
        assert!(api.api(Some(expired.clone())).is_err());
        api.oauth_client.should_succeed = true;
        assert!(api.api(Some(expired.clone())).is_ok());
    }

    #[test]
    fn api_throws_error_with_redirect_when_no_token() {
        let api: StravaApi<FakeOauth> = StravaApi::<FakeOauth>::new();
        assert!(api.api(None).is_err());
    }

    fn fake_oauth_token() -> OauthToken {
        OauthToken {
            id: 27,
            token: "tokenfromdb".to_string(),
            refresh: "refreshfromdb".to_string(),
            remote_athlete_id: 22,
            expires_at: Utc::now() + chrono::Duration::hours(1),
        }
    }

    fn fake_access_response() -> AccessTokenResponse {
        AccessTokenResponse {
            access: "tokenlol".to_string(),
            athlete: Some(fake_athlete_summary()),
            refresh: "refreshlol".to_string(),
            expires_at: Utc::now(),
        }
    }

    impl StravaApi<FakeOauth> {
        fn new() -> StravaApi<FakeOauth> {
            StravaApi {
                oauth_client: FakeOauth {
                    should_succeed: true,
                },
            }
        }
    }

    struct FakeOauth {
        pub should_succeed: bool,
    }

    impl FakeOauth {
        fn fake_net_response(&self) -> Result<AccessTokenResponse, StravaError> {
            if self.should_succeed {
                Ok(fake_access_response())
            } else {
                Err(StravaError::NetworkError(Box::new(
                    failure::err_msg("Exchange Code Failed").compat(),
                )))
            }
        }
    }

    impl OauthClient for FakeOauth {
        fn exchange_code(&self, _: &RedirectQuery) -> Result<AccessTokenResponse, StravaError> {
            self.fake_net_response()
        }

        fn refresh_token(&self, _: String) -> Result<AccessTokenResponse, StravaError> {
            self.fake_net_response()
        }

        fn get_authorization_url(&self) -> Url {
            "http://testwebsite.com".parse().unwrap()
        }
    }

    fn fake_athlete_summary() -> Summary {
        Summary {
            id: 0,
            first_name: "Fake".to_string(),
            last_name: "Athletelol".to_string(),
            city: "Townsburg".to_string(),
            country: "USA".to_string(),
        }
    }
}
