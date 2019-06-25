use failure::Fail;

#[derive(Debug, Fail)]
pub enum DataError {
    #[fail(display = "Sql Query Failed: {:?}", _0)]
    QueryError(#[fail(cause)] diesel::result::Error),

    #[fail(display = "Strava Api Returned Error: {:?}", _0)]
    StravaError(#[fail(cause)] strava::Error),
}
