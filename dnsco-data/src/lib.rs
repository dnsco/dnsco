#[macro_use]
extern crate diesel;

mod repos;
mod strava_api;

pub mod models;
pub mod schema;
pub use repos::Events as EventsRepo;
pub use strava_api::StravaApi;

pub mod database {
    use diesel::pg::PgConnection;
    use diesel::prelude::*;

    pub fn establish_connection(database_url: String) -> PgConnection {
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::models::NewActivity;
        use crate::repos::activities_repo::Repo;
        use std::time::UNIX_EPOCH;

        #[test]
        fn test_db() {
            let db_url = "postgres://dennis@localhost/dnsco";
            let connection = establish_connection(db_url.into());
            //            connection.test_transaction(|| -> QueryResult<()> {
            let repo = Repo {
                connection: &connection,
            };

            let new_id = std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time moves forward")
                .as_secs();

            let orig_count = repo.all().len();
            let mut activity = NewActivity {
                name: "Hey",
                description: None,
                distance: Some(9.4),
                remote_athlete_id: 0,
                remote_id: new_id as i32,
            };

            repo.upsert(&activity);
            assert_eq!(orig_count + 1, repo.all().len());
            activity.description = Some("WHAT");
            repo.upsert(&activity);
            let acts = repo.all();
            let new_description = acts
                .iter()
                .find(|a| a.remote_id == 0)
                .unwrap()
                .description
                .as_ref()
                .unwrap();

            assert_eq!("WHAT", new_description);
        }
    }
}
