use actix_web::{HttpResponse, ResponseError};
use failure::Fail;

pub type AppResult = Result<HttpResponse, AppError>;

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Strava Api Returned Error: {:?}", _0)]
    StravaError(#[fail(cause)] strava::Error),
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
                dbg!(e);
                HttpResponse::InternalServerError().body("Something Went Wrong")
            }
        }
    }
}
