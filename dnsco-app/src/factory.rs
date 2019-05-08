use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};

use dnsco_data::StravaApi;
use dnsco_service::config;
use dnsco_service::Webserver;

use crate::endpoints;

pub fn run_app(strava: StravaApi, urls: config::SiteUrls, port: u16) {
    let strava_api = Arc::new(Mutex::new(strava));

    let server_url = urls.site_url();
    println!("go to: {}", &server_url);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(Webserver::new(Arc::clone(&strava_api), urls.clone()))
            .service(web::resource("/").to(endpoints::index))
            .service(web::resource(urls.activities().path()).to(endpoints::activities))
            .service(web::resource(urls.oauth_redirect().path()).to(endpoints::oauth))
    })
    .bind(format!("0.0.0.0:{}", port))
    .unwrap()
    .run()
    .unwrap();
}
