use actix_web::{web, HttpResponse};

use dnsco_service::Webserver;
use strava::oauth::RedirectQuery as OauthQuery;

use crate::errors::{AppError, AppResult};
use crate::templates::TemplateResponse;

pub fn index(service: web::Data<Webserver>) -> AppResult {
    TemplateResponse::new(service.hello_world()).into()
}

pub fn activities(service: web::Data<Webserver>) -> AppResult {
    let activities = service.update_activities().map_err(AppError::StravaError)?;
    TemplateResponse::new(activities).into()
}

pub fn oauth(oauth_resp: web::Query<OauthQuery>, service: web::Data<Webserver>) -> AppResult {
    service
        .update_oauth_token(&oauth_resp)
        .map_err(AppError::StravaError)?;
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/activities")
        .finish())
}
