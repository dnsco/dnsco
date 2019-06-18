use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn create(database_url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::new(manager).unwrap();
        Self { pool }
    }

    pub fn get_connection(&self) -> Connection {
        self.pool.get().unwrap()
    }
}

#[cfg(testc)]
mod tests {
    use super::*;
    use crate::models::NewActivity;
    use crate::repos::activities_repo::Repo;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test_db() {
        let db = Database::create("postgres://dennis@localhost/dnsco".to_owned());
        let connection = db.get_connection();

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
