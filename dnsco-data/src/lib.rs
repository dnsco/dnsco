#[macro_use]
extern crate diesel;

pub mod repos;
mod strava_api;

pub mod database;
pub mod domains;
pub mod models;
pub mod schema;

pub use database::{Connection as DbConnection, Database};
pub use repos::Events as EventsRepo;
pub use strava_api::StravaApi;

pub use domains::activities::Repo as ActivitiesRepo;
pub use domains::oauth_tokens::Repo as OauthRepo;
