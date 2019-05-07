use crate::strava::models;
use failure::Fail;
use oauth2::basic::BasicErrorResponseType;
use oauth2::RequestTokenError;
use url::Url;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Strava Api Returned Error: {:?}", _0)]
    ApiError(models::ErrorResponse),

    #[fail(display = "Network Error: {:?}", _0)]
    NetworkError(#[fail(cause)] Box<failure::Fail>),

    #[fail(display = "No Strava Creds")]
    NoOauthToken(Url),

    #[fail(display = "Oauth Failure")]
    OauthAuthorizationError(#[fail(cause)] RequestTokenError<BasicErrorResponseType>),

    #[fail(display = "Failed to De/Serialize Strava Response")]
    Parse(#[fail(cause)] serde_json::Error),
}
