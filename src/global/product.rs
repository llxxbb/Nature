use dao::*;
use diesel::sqlite::SqliteConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

pub type CONN = SqliteConnection;

#[cfg(not(test))]
lazy_static! {
    pub static ref POOL :Pool<ConnectionManager<CONN>> = create_pool::<CONN>();
}
