use std::env;

use dnsco_app::{run_config, Config};

pub fn main() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_owned());
    let rust_log = env::var("RUST_LOG").unwrap_or(log_level);
    env::set_var("RUST_LOG", rust_log);
    pretty_env_logger::init();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse::<u16>()
        .ok()
        .unwrap();

    let conf = Config {
        db_url: env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://dennis@localhost/dnsco".to_owned()),
        strava_client_id: env::var("STRAVA_CLIENT_ID")
            .expect("Missing the STRAVA_CLIENT_ID environment variable."),
        strava_client_secret: env::var("STRAVA_CLIENT_SECRET")
            .expect("Missing the STRAVA_CLIENT_SECRET environment variable."),
        port,
        // localhost in host for urls/oauth callbacks, listent to 0.0.0.0 for production
        host: env::var("HOST").unwrap_or_else(|_| format!("localhost:{}", port)),
    };

    run_config(conf)
}
