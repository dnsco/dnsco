use oauth2::{Config, Token, TokenError};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use url::Url;

use serde::Deserialize;

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
    client = client.add_scope("activity:read_all");
    client = client.set_redirect_url(oauth_config.redirect_url.to_string());

    #[allow(deprecated)]
    let config = client.set_state("1917");
    config.authorize_url()
}

#[derive(Deserialize, Debug)]
pub struct RedirectQuery {
    code: String,
    state: String,
    scope: String,
}

pub fn redirect_callback(query: &RedirectQuery, config: &ClientConfig) -> String {
    let code = query.code.clone();
    let client = oauth2_client(&config);
    if let Ok(code) = client.exchange_code(code) {
        return format!("{:?}", code);
    }

    return "Nope".to_owned();
}

fn oauth2_client(oauth_config: &ClientConfig) -> Config {
    Config::new(
        oauth_config.client_id.clone(),
        oauth_config.client_secret.clone(),
        AUTH_URL,
        TOKEN_URL,
    )
}
