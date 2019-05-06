use actix_web::{web, App, HttpServer, Responder};

mod service;
mod strava;
use service::Webserver;
use std::env;
use std::sync::Arc;
use strava::OauthRedirectQuery;

pub fn main() {
    let strava_client_id =
        env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
    let strava_client_secret = env::var("STRAVA_CLIENT_SECRET")
        .expect("Missing the STRAVA_CLIENT_SECRET environment variable.");
    let strava_oauth_redirct_url =
        env::var("STRAVA_OAUTH_REDIRECT").unwrap_or("http://localhost:8080".to_owned());
    let strava_access_token = env::var("STRAVA_OAUTH_TOKEN").ok();

    let host = env::var("HOST").unwrap_or("localhost".to_owned());
    let port = env::var("PORT")
        .unwrap_or("8080".to_owned())
        .parse::<u16>()
        .ok();

    let scheme = "http";
    let host_with_scheme = format!("{}://{}", scheme, host);
    let mut url = url::Url::parse(&host_with_scheme).unwrap();
    url.set_port(port).unwrap();

    let activities_url = url.join("/activities").unwrap();
    let oauth_redirect_url = url.join("/oauth").unwrap();

    let oauth_config = strava::OauthConfig {
        client_id: strava_client_id,
        client_secret: strava_client_secret,
        redirect_url: oauth_redirect_url.clone(),
    };

    let strava_api = Arc::new(strava::authenticate(
        strava_access_token,
        oauth_config.clone(),
    ));

    println!("go to: {}", activities_url);

    HttpServer::new(move || {
        App::new()
            .data(Webserver::new(strava_api.clone(), oauth_config.clone()))
            .service(web::resource(activities_url.path()).to(activities))
            .service(web::resource(oauth_redirect_url.path()).to(oauth))
    })
    .bind(&url)
    .unwrap()
    .run()
    .unwrap()
}

#[derive(Debug)]
struct TemplateResponse {
    page: service::IndexResponse,
}

fn activities(service: web::Data<Webserver>) -> impl Responder {
    service.activities().map_err(log_and_convert_error)
}

fn oauth(
    oauth_resp: web::Query<OauthRedirectQuery>,
    service: web::Data<Webserver>,
) -> impl Responder {
    strava::oauth_redirect_callback(&oauth_resp, &service.oauth_config)
}

fn log_and_convert_error(error: String) -> actix_web::Error {
    dbg!(error);
    actix_web::error::ErrorExpectationFailed("Something went wrong")
}
