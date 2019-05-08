use std::env;

use dnsco_app::factory::run_app;
use dnsco_service::config;

pub fn main() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_owned());
    let rust_log = env::var("RUST_LOG").unwrap_or(log_level);
    env::set_var("RUST_LOG", rust_log);
    env_logger::init();

    let strava_client_id =
        env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
    let strava_client_secret = env::var("STRAVA_CLIENT_SECRET")
        .expect("Missing the STRAVA_CLIENT_SECRET environment variable.");
    let strava_access_token = env::var("STRAVA_OAUTH_TOKEN").ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse::<u16>()
        .ok()
        .unwrap();

    // localhost in host for urls/oauth callbacks, listent to 0.0.0.0 for production
    let host = env::var("HOST").unwrap_or_else(|_| format!("localhost:{}", port));
    let urls = config::SiteUrls::new(host);

    let strava_api = dnsco_data::StravaApi::new(
        strava_access_token,
        strava::oauth::ClientConfig {
            client_id: strava_client_id,
            client_secret: strava_client_secret,
            redirect_url: urls.oauth_redirect(),
        },
    );

    run_app(strava_api, urls, port)
}
