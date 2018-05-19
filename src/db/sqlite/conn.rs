extern crate dotenv;

use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use self::dotenv::dotenv;
use std::env;
use std::sync::Mutex;

lazy_static! {
    pub static ref CONN :Mutex<SqliteConnection>  = Mutex::new(establish_connection());
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

//lazy_static! {
//    pub static ref POOL : Pool<ConnectionManager<SqliteConnection>> = make_db_connection_pool();
//}

//impl From<Error> for NatureError {
//    fn from(err: Error) -> Self {
//        NatureError::R2D2Error(err.to_string())
//    }
//}

//fn make_db_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
//    dotenv().ok();
//
//    let database_url = env::var("DATABASE_URL")
//        .expect("DATABASE_URL must be set");
//
//    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
//    Pool::builder().build(manager).expect("Failed to create pool.")
//}
//
//pub struct DBPool;
//
//impl DBPool {

// /// 使用说明：
// ///
// /// ```rust
// /// use std::ops::Deref;
// /// let conn = DBPool::get_connection()?;
// /// conn.deref()
// /// ```
//    pub fn get_connection() -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
//        match POOL.clone().get() {
//            Err(err) => Err(NatureError::from(err)),
//            Ok(conn) => Ok(conn),
//        }
//    }
//}




