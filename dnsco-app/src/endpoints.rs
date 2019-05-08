use actix_web::{web, HttpResponse, Responder};

use crate::errors::{AppError, AppResult};
use crate::templates::into_response;
use dnsco_service::Webserver;
use strava::oauth::RedirectQuery as OauthQuery;

pub fn index(service: web::Data<Webserver>) -> impl Responder {
    into_response(service.hello_world())
}

pub fn activities(service: web::Data<Webserver>) -> AppResult {
    let activities = service.activities().map_err(AppError::StravaError)?;
    Ok(HttpResponse::Ok().body(activities))
}

pub fn oauth(oauth_resp: web::Query<OauthQuery>, service: web::Data<Webserver>) -> AppResult {
    service
        .update_oauth_token(&oauth_resp)
        .map_err(AppError::StravaError)?;
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/activities")
        .finish())
}
