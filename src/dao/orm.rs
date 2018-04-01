extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::sqlite::SqliteConnection;
use self::dotenv::dotenv;
use self::r2d2::{ManageConnection, Pool};
use self::r2d2_diesel::ConnectionManager;
use std::env;

pub struct DieselService {
    pub pool: Option<Pool<ConnectionManager<SqliteConnection>>>
}

impl DieselService {
    fn init() -> Pool<ConnectionManager<SqliteConnection>> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
    }

    pub fn get_connection(&self) -> Pool<ConnectionManager<SqliteConnection>> {
        self.pool.get_or_insert(Self::init()).clone()
    }
}




