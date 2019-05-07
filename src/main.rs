use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::sync::{Arc, Mutex};

use service::Webserver;
use url::Url;

mod service;
mod strava;

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

    let urls = SiteUrls::new(host);

    let strava_api = Arc::new(Mutex::new(strava::AuthedApi::new(
        strava_access_token,
        strava::oauth::ClientConfig {
            client_id: strava_client_id,
            client_secret: strava_client_secret,
            redirect_url: urls.oauth_redirect(),
        },
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
    .bind(server_listen)
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

fn into_response<T: askama::Template>(template: T) -> Result<HttpResponse, actix_web::Error> {
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
    pub fn new(host: String) -> Self {
        let host_with_scheme = format!("http://{}", host);

        Self {
            base: url::Url::parse(&host_with_scheme).unwrap(),
        }
    }

    pub fn activities(&self) -> Url {
        self.base.join("/activities").unwrap()
    }

    pub fn oauth_redirect(&self) -> Url {
        self.base.join("/oauth").unwrap()
    }
}
