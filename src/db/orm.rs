extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::connection::Connection;
use self::dotenv::dotenv;
use self::r2d2::Pool;
use self::r2d2_diesel::ConnectionManager;
use std::env;


pub fn create_pool<T: 'static + Connection>() -> Pool<ConnectionManager<T>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<T>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}





