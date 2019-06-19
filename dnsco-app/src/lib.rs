use actix_web::error::BlockingError;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use failure::Fail;
use futures::Future;
use log::error;

use std::sync::{Arc, Mutex};

use dnsco_data::{Database, StravaApi};
use strava::oauth::RedirectQuery as OauthQuery;

mod app_service;

pub mod config;
pub mod domains;

pub fn run(db: Database, strava: StravaApi, urls: config::SiteUrls, port: u16) {
    let database = Arc::new(db);
    let strava_api = Arc::new(Mutex::new(strava));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(app_service::Service::new(
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

pub fn index(service: web::Data<app_service::Service>) -> AppResult {
    TemplateResponse::new(service.hello_world()).into()
}

pub fn activities(
    service: web::Data<app_service::Service>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    render_template(move || service.activities())
}

fn render_template<F, T>(f: F) -> impl Future<Item = HttpResponse, Error = actix_web::Error>
where
    T: Template + Send + 'static,
    F: FnOnce() -> Result<T, AppError> + Send + 'static,
{
    web::block(f)
        .map(|t| AppResult::from(TemplateResponse::new(t)))
        .then(|result| match result {
            Ok(Ok(temp)) => temp,
            err => err.into(),
        })
}

pub fn update_activities(
    service: web::Data<app_service::Service>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let redirect_path = service.urls.activities().path().to_owned();

    web::block(move || service.update_activities().map_err(AppError::StravaError)).then(|res| {
        match res {
            Ok(_) => HttpResponse::Found()
                .header(http::header::LOCATION, redirect_path)
                .finish(),
            Err(e) => AppError::from(e).error_response(),
        }
    })
}

pub fn oauth(
    oauth_resp: web::Query<OauthQuery>,
    service: web::Data<app_service::Service>,
) -> AppResult {
    let redirect_path = service.urls.update_activities().path().to_owned();
    service
        .update_oauth_token(&oauth_resp)
        .map_err(AppError::StravaError)?;
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, redirect_path)
        .finish())
}

pub type AppResult = Result<HttpResponse, AppError>;

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Strava Api Returned Error: {:?}", _0)]
    StravaError(#[fail(cause)] strava::Error),

    #[fail(display = "Issue Rendering Template: {:?}", _0)]
    TemplateError(#[fail(cause)] Box<Fail>),

    #[fail(display = "Threadpool is gone")]
    ThreadCanceled,
}

impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> AppError {
        match e {
            BlockingError::Error(e) => e,
            BlockingError::Canceled => AppError::ThreadCanceled,
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::StravaError(strava::Error::NoOauthToken(redirect_url)) => {
                HttpResponse::Found()
                    .header(http::header::LOCATION, redirect_url.to_string())
                    .finish()
            }
            e => {
                error!("Unhandled Error: {}", e);
                HttpResponse::InternalServerError().body("Something Went Wrong")
            }
        }
    }
}

pub struct TemplateResponse<T: Template>(T);

impl<T: Template> TemplateResponse<T> {
    pub fn new(template: T) -> Self {
        TemplateResponse(template)
    }
}

impl<T: Template> From<TemplateResponse<T>> for AppResult {
    fn from(t: TemplateResponse<T>) -> AppResult {
        let rendered =
            t.0.render()
                .map_err(|e| AppError::TemplateError(Box::new(e)))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
    }
}
