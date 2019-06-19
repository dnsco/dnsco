pub mod app;
pub mod service;
use failure::Fail;

pub use app::{run_config, Config};

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Sql Query Failed: {:?}", _0)]
    QueryError(#[fail(cause)] diesel::result::Error),

    #[fail(display = "Strava Api Returned Error: {:?}", _0)]
    StravaError(#[fail(cause)] strava::Error),

    #[fail(display = "Issue Rendering Template: {:?}", _0)]
    TemplateError(#[fail(cause)] Box<Fail>),

    #[fail(display = "Threadpool is gone")]
    ThreadCanceled,
}
