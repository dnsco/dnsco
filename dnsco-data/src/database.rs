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
