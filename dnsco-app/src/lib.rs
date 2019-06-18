use actix_web::error::BlockingError;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use log::error;

use std::sync::{Arc, Mutex};

use dnsco_data::{Database, StravaApi};

use strava::oauth::RedirectQuery as OauthQuery;

mod errors;
mod templates;
mod webserver;

use errors::{AppError, AppResult};
use futures::Future;
use templates::TemplateResponse;
use webserver::Service;

pub mod config;

pub fn run(db: Database, strava: StravaApi, urls: config::SiteUrls, port: u16) {
    let database = Arc::new(db);
    let strava_api = Arc::new(Mutex::new(strava));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(Service::new(
                Arc::clone(&database),
                Arc::clone(&strava_api),
                urls.clone(),
            ))
            .service(web::resource("/").to(index))
            .route(urls.activities().path(), web::get().to_async(activities))
            .route(
                urls.update_activities().path(),
                web::get().to_async(update_activities),
            )
            .service(web::resource(urls.oauth_redirect().path()).to(oauth))
    })
    .bind(format!("0.0.0.0:{}", port))
    .unwrap()
    .run()
    .unwrap();
}

pub fn index(service: web::Data<Service>) -> AppResult {
    TemplateResponse::new(service.hello_world()).into()
}

pub fn activities(
    service: web::Data<Service>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || service.activities()).then(|res| match res {
        Ok(acts) => HttpResponse::Ok().body(acts),
        Err(_) => HttpResponse::InternalServerError().into(),
    })
}

pub fn update_activities(
    service: web::Data<Service>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || service.update_activities().map_err(AppError::StravaError)).then(|res| {
        match res {
            Ok(activities) => HttpResponse::Ok().body(activities.render().unwrap()),
            Err(e) => match e {
                BlockingError::Error(ref error) => handle_app_error(error),
                _ => HttpResponse::InternalServerError().into(),
            },
        }
    })
}

pub fn oauth(oauth_resp: web::Query<OauthQuery>, service: web::Data<Service>) -> AppResult {
    let redirect_path = service.urls.update_activities().path().to_owned();
    service
        .update_oauth_token(&oauth_resp)
        .map_err(AppError::StravaError)?;
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, redirect_path)
        .finish())
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        handle_app_error(self)
    }
}

fn handle_app_error(error: &AppError) -> HttpResponse {
    match error {
        AppError::StravaError(strava::Error::NoOauthToken(redirect_url)) => HttpResponse::Found()
            .header(http::header::LOCATION, redirect_url.to_string())
            .finish(),
        e => {
            error!("Unhandled Error: {}", e);
            HttpResponse::InternalServerError().body("Something Went Wrong")
        }
    }
}
