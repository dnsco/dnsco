use crate::strava::models;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Strava Api Returned Error: {:?}", _0)]
    ApiError(models::ErrorResponse),

    #[fail(display = "Network Error: {:?}", _0)]
    NetworkError(#[fail(cause)] Box<failure::Fail>),
}
