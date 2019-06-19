use oauth2::basic::BasicTokenType;
use oauth2::prelude::*;
use oauth2::{
    AccessToken, AuthType, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, RefreshToken, RequestTokenError, Scope, StandardTokenResponse, TokenResponse,
    TokenUrl,
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

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: url::Url,
}

#[derive(Deserialize, Debug)]
pub struct RedirectQuery {
    code: String,
    state: String,
    scope: String,
}

#[derive(Debug)]
pub struct AccessTokenResponse {
    pub access: AccessToken,
    pub athlete: models::athlete::Summary,
    pub refresh: RefreshToken,
}

impl AccessTokenResponse {
    pub fn oauth_token(&self) -> String {
        self.access.secret().clone()
    }

    pub fn refresh_token(&self) -> String {
        self.refresh.secret().clone()
    }
}

impl From<OauthResponse> for AccessTokenResponse {
    fn from(resp: OauthResponse) -> Self {
        Self {
            access: resp.access_token().to_owned(),
            refresh: resp.refresh_token().unwrap().to_owned(),
            athlete: resp.extra_fields().to_owned().athlete,
        }
    }
}

pub fn get_authorization_url(oauth_config: &ClientConfig) -> Url {
    let mut client = oauth2_client(oauth_config);
    let redirect_url = RedirectUrl::new(oauth_config.redirect_url.clone());
    let scope = Scope::new("activity:read_all".to_owned());

    client = client.add_scope(scope);
    client = client.set_redirect_url(redirect_url);
    client.authorize_url(CsrfToken::new_random).0
}

pub fn redirect_callback(
    query: &RedirectQuery,
    config: &ClientConfig,
) -> Result<AccessTokenResponse, Error> {
    let code = AuthorizationCode::new(query.code.clone());
    match oauth2_client(&config).exchange_code(code) {
        Ok(resp) => Ok(resp.into()),
        Err(err) => Err(err.into()),
    }
}

fn oauth2_client(oauth_config: &ClientConfig) -> OauthClient {
    let client_id = ClientId::new(oauth_config.client_id.clone());
    let client_secret = ClientSecret::new(oauth_config.client_secret.clone());
    let auth_url = AuthUrl::new(Url::parse(AUTH_URL).expect("Invalid AuthUrl"));
    let token_url = TokenUrl::new(Url::parse(TOKEN_URL).expect("Invalid TokenUrl"));

    OauthClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_auth_type(AuthType::RequestBody)
}

type OauthResponse = StandardTokenResponse<HasAthlete, BasicTokenType>;
type OauthClient = Client<models::ErrorResponse, OauthResponse, BasicTokenType>;

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
struct HasAthlete {
    athlete: models::athlete::Summary,
}

impl oauth2::ExtraTokenFields for HasAthlete {}

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
