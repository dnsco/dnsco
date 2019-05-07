use oauth2::basic::BasicClient;
use oauth2::prelude::{NewType, SecretNewType};
use oauth2::{
    AccessToken, AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use url::Url;

use serde::Deserialize;

use crate::strava::errors::Error as StravaError;

#[derive(Debug, Clone)]
pub struct OauthToken(pub String);

const AUTH_URL: &str = "https://www.strava.com/oauth/authorize";
const TOKEN_URL: &str = "https://www.strava.com/oauth/token";

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: url::Url,
}

pub fn get_authorization_url(oauth_config: &ClientConfig) -> Url {
    let mut client = oauth2_client(oauth_config);
    let redirect_url = RedirectUrl::new(oauth_config.redirect_url.clone());
    let scope = Scope::new("activity:read_all".to_owned());

    client = client.add_scope(scope);
    client = client.set_redirect_url(redirect_url);
    client.authorize_url(CsrfToken::new_random).0
}

#[derive(Deserialize, Debug)]
pub struct RedirectQuery {
    code: String,
    state: String,
    scope: String,
}

#[derive(Debug)]
pub struct AccessTokenResponse(pub AccessToken, RefreshToken);
impl AccessTokenResponse {
    pub fn oauth_token(&self) -> String {
        self.0.secret().clone()
    }

    //    pub fn refresh_token(&self) -> String {
    //        self.1.secret().clone()
    //    }
}

pub fn redirect_callback(
    query: &RedirectQuery,
    config: &ClientConfig,
) -> Result<AccessTokenResponse, StravaError> {
    let code = AuthorizationCode::new(query.code.clone());
    match oauth2_client(&config).exchange_code(code) {
        Ok(resp) => Ok(AccessTokenResponse(
            resp.access_token().clone(),
            resp.refresh_token().unwrap().clone(),
        )),
        Err(e) => Err(StravaError::OauthAuthorizationError(e)),
    }
}

fn oauth2_client(oauth_config: &ClientConfig) -> BasicClient {
    let client_id = ClientId::new(oauth_config.client_id.clone());
    let client_secret = ClientSecret::new(oauth_config.client_secret.clone());
    let auth_url = AuthUrl::new(Url::parse(AUTH_URL).expect("Invalid AuthUrl"));
    let token_url = TokenUrl::new(Url::parse(TOKEN_URL).expect("Invalid TokenUrl"));

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_auth_type(AuthType::RequestBody)
}
