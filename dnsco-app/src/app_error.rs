use dnsco_data::DataError;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Data Layer Failure: {:?}", _0)]
    DataError(#[fail(cause)] DataError),

    #[fail(display = "Sql Query Failed: {:?}", _0)]
    QueryError(#[fail(cause)] diesel::result::Error),

    #[fail(display = "Strava Api Returned Error: {:?}", _0)]
    StravaError(#[fail(cause)] strava::Error),

    #[fail(display = "Issue Rendering Template: {:?}", _0)]
    TemplateError(#[fail(cause)] Box<Fail>),

    #[fail(display = "Threadpool is gone")]
    ThreadCanceled,
}

impl From<DataError> for AppError {
    fn from(err: DataError) -> Self {
        AppError::DataError(err)
    }
}
