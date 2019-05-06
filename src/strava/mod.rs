mod api;
mod errors;
mod models;
mod oauth;

pub use api::Api;

pub use oauth::{
    redirect_callback as oauth_redirect_callback, ClientConfig as OauthConfig,
    RedirectQuery as OauthRedirectQuery,
};

pub fn authenticate(access_token: Option<String>, oauth_config: oauth::ClientConfig) -> Api {
    let token = oauth::OauthToken(
        access_token.unwrap_or_else(|| oauth::get_authorization_url(&oauth_config).to_string()),
    );
    dbg!(&token);

    Api::new(token)
}
