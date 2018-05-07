extern crate dotenv;

use diesel::sqlite::SqliteConnection;
use global::*;
use global::error::NatureError;
use r2d2::{Error, Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use self::dotenv::dotenv;
use std::env;



lazy_static! {
    pub static ref POOL : Pool<ConnectionManager<SqliteConnection>> = make_db_connection_pool();
}

impl From<Error> for NatureError {
    fn from(err: Error) -> Self {
        NatureError::R2D2Error(err.to_string())
    }
}

fn make_db_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

pub struct DBPool;

impl DBPool {
    pub fn get_connection() -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        match POOL.clone().get() {
            Err(err) => Err(NatureError::from(err)),
            Ok(conn) => Ok(conn),
        }
    }
}

