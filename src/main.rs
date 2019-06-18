use std::env;

use dnsco_app::run;
use dnsco_service::config;

pub fn main() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_owned());
    let rust_log = env::var("RUST_LOG").unwrap_or(log_level);
    env::set_var("RUST_LOG", rust_log);
    pretty_env_logger::init();

    let db_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://dennis@localhost/dnsco".to_owned());

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

    let site_urls = config::SiteUrls::new(host);
    let pool = dnsco_data::Database::create(db_url);
    let strava_api = dnsco_data::StravaApi::new(
        strava_access_token,
        strava::oauth::ClientConfig {
            client_id: strava_client_id,
            client_secret: strava_client_secret,
            redirect_url: site_urls.oauth_redirect(),
        },
    );

    run(pool, strava_api, site_urls, port)
}
