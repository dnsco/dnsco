use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::sync::{Arc, Mutex};

use askama::Template;
use service::Webserver;
use url::Url;

mod service;
mod strava;

pub fn main() {
    let strava_client_id =
        env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
    let strava_client_secret = env::var("STRAVA_CLIENT_SECRET")
        .expect("Missing the STRAVA_CLIENT_SECRET environment variable.");
    let strava_access_token = env::var("STRAVA_OAUTH_TOKEN").ok();

    let host = env::var("HOST").unwrap_or("localhost".to_owned());
    let port = env::var("PORT")
        .unwrap_or("8080".to_owned())
        .parse::<u16>()
        .ok();

    let urls = SiteUrls::new(host, port);

    let oauth_config = strava::oauth::ClientConfig {
        client_id: strava_client_id,
        client_secret: strava_client_secret,
        redirect_url: urls.oauth_redirect(),
    };

    let strava_api = Arc::new(Mutex::new(strava::AuthedApi::new(
        strava_access_token,
        oauth_config.clone(),
    )));

    let server_url = urls.base.clone();

    println!("go to: {}", &server_url);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(Webserver::new(strava_api.clone(), urls.clone()))
            .service(web::resource("/").to(index))
            .service(web::resource(urls.activities().path()).to(activities))
            .service(web::resource(urls.oauth_redirect().path()).to(oauth))
    })
    .bind(server_url)
    .unwrap()
    .run()
    .unwrap()
}

fn index(service: web::Data<Webserver>) -> impl Responder {
    into_response(service.hello_world())
}

fn activities(service: web::Data<Webserver>) -> HttpResponse {
    let strava = service.strava_api.lock().unwrap();
    if let Ok(api) = strava.api() {
        match service.activities(&api) {
            Ok(activities) => HttpResponse::Ok().body(activities),
            Err(err) => log_and_convert_error(err),
        }
    } else {
        HttpResponse::Found()
            .header(http::header::LOCATION, strava.auth_url().to_string())
            .finish()
            .into_body()
    }
}

fn oauth(
    oauth_resp: web::Query<strava::oauth::RedirectQuery>,
    service: web::Data<Webserver>,
) -> impl Responder {
    let mut strava = service.strava_api.lock().unwrap();

    if let Ok(resp) = strava.parsed_oauth_response(&oauth_resp) {
        strava.set_tokens(&resp);

        HttpResponse::Found()
            .header(http::header::LOCATION, "/activities")
            .finish()
    } else {
        HttpResponse::InternalServerError().body("Sadness")
    }
}

fn log_and_convert_error(error: String) -> HttpResponse {
    dbg!(error);
    HttpResponse::InternalServerError().body("nope")
}

fn into_response<T: Template>(template: T) -> Result<HttpResponse, actix_web::Error> {
    let rsp = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template parsing error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rsp))
}

#[derive(Clone)]
pub struct SiteUrls {
    base: Url,
}

impl SiteUrls {
    pub fn new(host: String, port: Option<u16>) -> Self {
        let host_with_scheme = format!("http://{}", host);
        let mut base = url::Url::parse(&host_with_scheme).unwrap();
        base.set_port(port).unwrap();

        Self { base }
    }

    pub fn activities(&self) -> Url {
        self.base.join("/activities").unwrap()
    }

    pub fn oauth_redirect(&self) -> Url {
        self.base.join("/oauth").unwrap()
    }
}
