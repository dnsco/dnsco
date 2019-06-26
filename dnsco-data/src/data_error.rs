use diesel::result::Error as DieselError;
use failure::Fail;
use url::Url;

pub type DataResult<T> = Result<T, DataError>;

#[derive(Debug, Fail)]
pub enum DataError {
    #[fail(display = "Not Authenticated, redirect_to {}", _0)]
    NotAuthenticated(Url),

    #[fail(display = "Sql Query Failed: {:?}", _0)]
    QueryError(#[fail(cause)] DieselError),

    #[fail(display = "Strava Api Returned Error: {:?}", _0)]
    StravaError(#[fail(cause)] strava::Error),
}

impl From<strava::Error> for DataError {
    fn from(strava_error: strava::Error) -> Self {
        match strava_error {
            strava::Error::NoOauthToken(url) => DataError::NotAuthenticated(url),
            err => DataError::StravaError(err),
        }
    }
}

impl From<DieselError> for DataError {
    fn from(diesel_error: DieselError) -> Self {
        DataError::QueryError(diesel_error)
    }
}
