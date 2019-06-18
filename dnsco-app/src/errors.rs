use actix_web::HttpResponse;
use failure::Fail;

pub type AppResult = Result<HttpResponse, AppError>;

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Strava Api Returned Error: {:?}", _0)]
    StravaError(#[fail(cause)] strava::Error),

    #[fail(display = "Issue Rendering Template: {:?}", _0)]
    TemplateError(#[fail(cause)] Box<Fail>),
}
