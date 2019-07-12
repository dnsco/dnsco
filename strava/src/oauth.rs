use chrono::{DateTime, TimeZone, Utc};
use oauth2::basic::BasicTokenType;
use oauth2::prelude::*;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, Client as Oauth2Client, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, RefreshToken, RequestTokenError, Scope, StandardTokenResponse,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{models, Error};

#[derive(Clone)]
pub struct OauthToken(pub String);

impl std::fmt::Debug for OauthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OauthToken([redacted])")
    }
}

const AUTH_URL: &str = "https://www.strava.com/oauth/authorize";
const TOKEN_URL: &str = "https://www.strava.com/oauth/token";

pub trait OauthClient {
    fn exchange_code(&self, query: &RedirectQuery) -> Result<AccessTokenResponse, Error>;
    fn refresh_token(&self, token: String) -> Result<AccessTokenResponse, Error>;
    fn get_authorization_url(&self) -> Url;
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: url::Url,
}

pub struct StravaOauth<'a> {
    config: &'a ClientConfig,
}

impl<'a> StravaOauth<'a> {
    pub fn new(config: &'a ClientConfig) -> Self {
        Self { config }
    }

    fn oauth2_client(&self) -> Client {
        Client::new(
            ClientId::new(self.config.client_id.clone()),
            Some(ClientSecret::new(self.config.client_secret.clone())),
            AuthUrl::new(Url::parse(AUTH_URL).expect("Invalid AuthUrl")),
            Some(TokenUrl::new(
                Url::parse(TOKEN_URL).expect("Invalid TokenUrl"),
            )),
        )
        .set_auth_type(AuthType::RequestBody)
    }
}

impl<'a> OauthClient for StravaOauth<'a> {
    fn exchange_code(&self, query: &RedirectQuery) -> Result<AccessTokenResponse, Error> {
        let code = AuthorizationCode::new(query.code.clone());
        match self.oauth2_client().exchange_code(code) {
            Ok(resp) => Ok(resp.into()),
            Err(err) => Err(err.into()),
        }
    }

    fn refresh_token(&self, token: String) -> Result<AccessTokenResponse, Error> {
        let refresh = RefreshToken::new(token);
        match self.oauth2_client().exchange_refresh_token(&refresh) {
            Ok(resp) => Ok(resp.into()),
            Err(e) => Err(e.into()),
        }
    }

    fn get_authorization_url(&self) -> Url {
        let mut client = self.oauth2_client();
        let redirect_url = RedirectUrl::new(self.config.redirect_url.clone());
        let scope = Scope::new("activity:read_all".to_owned());

        client = client.add_scope(scope);
        client = client.set_redirect_url(redirect_url);
        client.authorize_url(CsrfToken::new_random).0
    }
}

type OauthResponse = StandardTokenResponse<OauthResponseFields, BasicTokenType>;
type Client = Oauth2Client<models::ErrorResponse, OauthResponse, BasicTokenType>;

#[derive(Deserialize, Debug)]
pub struct RedirectQuery {
    code: String,
    state: String,
    scope: String,
}

#[derive(Debug)]
pub struct AccessTokenResponse {
    pub access: String,
    pub athlete: Option<models::athlete::Summary>,
    pub refresh: String,
    pub expires_at: DateTime<Utc>,
}

impl From<OauthResponse> for AccessTokenResponse {
    fn from(resp: OauthResponse) -> Self {
        let extra = resp.extra_fields().to_owned();
        Self {
            access: resp.access_token().secret().to_owned(),
            refresh: resp.refresh_token().unwrap().secret().to_owned(),
            athlete: extra.athlete,
            expires_at: Utc.timestamp(extra.expires_at, 0),
        }
    }
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub struct OauthResponseFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    athlete: Option<models::athlete::Summary>,
    expires_at: i64,
}

impl oauth2::ExtraTokenFields for OauthResponseFields {}

impl oauth2::ErrorResponseType for models::ErrorResponse {}

impl From<RequestTokenError<models::ErrorResponse>> for Error {
    fn from(error: RequestTokenError<models::ErrorResponse>) -> Self {
        match error {
            oauth2::RequestTokenError::Parse(error, resp) => {
                match serde_json::from_slice::<models::ErrorResponse>(&resp) {
                    Ok(e) => Error::OauthAuthorizationError(e),
                    Err(_) => Error::Parse(error, Some(resp)),
                }
            }

            e => Error::NetworkError(Box::new(e)),
        }
    }
}
