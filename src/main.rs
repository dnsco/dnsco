use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, ResponseError};
use std::env;
use std::sync::{Arc, Mutex};

use service::Webserver;

mod config;
mod service;
mod strava;
mod templates;

pub fn main() {
    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".to_owned());
    let rust_log = env::var("RUST_LOG").unwrap_or(log_level);
    env::set_var("RUST_LOG", rust_log);
    env_logger::init();

    let strava_client_id =
        env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
    let strava_client_secret = env::var("STRAVA_CLIENT_SECRET")
        .expect("Missing the STRAVA_CLIENT_SECRET environment variable.");
    let strava_access_token = env::var("STRAVA_OAUTH_TOKEN").ok();

    let port = env::var("PORT")
        .unwrap_or("8080".to_owned())
        .parse::<u16>()
        .ok();

    // localhost in host for urls/oauth callbacks, listent to 0.0.0.0 for production
    let host = env::var("HOST").unwrap_or(format!("localhost:{}", port.unwrap()));
    let server_listen = format!("0.0.0.0:{}", port.unwrap());

    let urls = config::SiteUrls::new(host);

    let strava_api = Arc::new(Mutex::new(service::StravaApi::new(
        strava_access_token,
        strava::oauth::ClientConfig {
            client_id: strava_client_id,
            client_secret: strava_client_secret,
            redirect_url: urls.oauth_redirect(),
        },
    )));

    let server_url = urls.site_url();

    println!("go to: {}", &server_url);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(Webserver::new(strava_api.clone(), urls.clone()))
            .service(web::resource("/").to(index))
            .service(web::resource(urls.activities().path()).to(activities))
            .service(web::resource(urls.oauth_redirect().path()).to(oauth))
    })
    .bind(server_listen)
    .unwrap()
    .run()
    .unwrap()
}

fn index(service: web::Data<Webserver>) -> impl Responder {
    into_response(service.hello_world())
}

fn activities(service: web::Data<Webserver>) -> Result<HttpResponse, strava::Error> {
    let activities = service.activities()?;
    Ok(HttpResponse::Ok().body(activities))
}

fn oauth(
    oauth_resp: web::Query<strava::oauth::RedirectQuery>,
    service: web::Data<Webserver>,
) -> Result<HttpResponse, strava::Error> {
    service.update_oauth_token(&oauth_resp)?;
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/activities")
        .finish())
}

fn into_response<T: askama::Template>(template: T) -> Result<HttpResponse, actix_web::Error> {
    let rsp = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template parsing error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rsp))
}

impl ResponseError for strava::Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            strava::Error::NoOauthToken(redirect_url) => HttpResponse::Found()
                .header(http::header::LOCATION, redirect_url.to_string())
                .finish(),
            e => {
                dbg!(e);
                HttpResponse::InternalServerError().body("Something Went Wrong")
            }
        }
    }
}
