use actix_web::{web, App, HttpServer, Responder, ResponseError};

mod service;
mod strava;
use service::Webserver;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

pub fn main() {
    let strava_client_id =
        env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
    let strava_client_secret = env::var("STRAVA_CLIENT_SECRET")
        .expect("Missing the STRAVA_CLIENT_SECRET environment variable.");
    let strava_oauth_redirct_url =
        env::var("STRAVA_OAUTH_REDIRECT").unwrap_or("http://localhost:8080".to_owned());
    let strava_access_token = env::var("STRAVA_OAUTH_TOKEN").ok();

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address");

    println!("Listening on http://{}", addr);

    let strava_api = Arc::new(strava::authenticate(
        strava_access_token,
        strava::OauthConfig {
            client_id: strava_client_id,
            client_secret: strava_client_secret,
            redirect_url: strava_oauth_redirct_url,
        },
    ));

    println!("go to http://localhost:8080/activities");

    HttpServer::new(move || {
        App::new()
            .data(Webserver::new(strava_api.clone()))
            .service(web::resource("/activities").to(activities))
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap()
}

#[derive(Debug)]
struct TemplateResponse {
    page: service::IndexResponse,
}

fn activities(service: web::Data<Webserver>) -> impl Responder {
    service
        .activities()
        .map_err(|e| actix_web::error::ErrorExpectationFailed(e))
}
