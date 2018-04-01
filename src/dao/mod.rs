extern crate r2d2_diesel;

pub use dao::instance::*;
use dao::orm::DieselService;
pub use dao::thing::*;
use diesel::sqlite::SqliteConnection;
use self::r2d2_diesel::ConnectionManager;


pub type CONN = ConnectionManager<SqliteConnection>;

pub mod instance;
pub mod transmit;
pub mod thing;
pub mod sqlite;
pub mod orm;


