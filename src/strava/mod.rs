mod api;
mod errors;
mod models;
mod oauth;

pub use api::Api;
pub use oauth::ClientConfig as OauthConfig;

pub fn authenticate(access_token: Option<String>, oauth_config: oauth::ClientConfig) -> Api {
    let token = oauth::OauthToken(
        access_token.unwrap_or_else(|| oauth::oauth_dance(oauth_config).unwrap().access_token),
    );

    Api::new(token)
}
