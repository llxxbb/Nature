use diesel::result::*;
use global::*;
pub use self::conn::*;
pub use self::models::*;
pub use self::table::*;

impl From<Error> for NatureError {
    // TODO put aside because can't find diesel's Timeout Error
    fn from(err: Error) -> Self {
        match err {
            Error::DatabaseError(kind, info) => {
                match kind {
                    DatabaseErrorKind::UniqueViolation => NatureError::DaoDuplicated,
                    DatabaseErrorKind::__Unknown => NatureError::DaoEnvironmentError(format!("{:?}", info)),
                    _ => NatureError::DaoLogicalError(format!("{:?}", info)),
                }
            }
            _ => NatureError::DaoLogicalError(err.to_string()),
        }
    }
}

mod table;

mod conn;

mod models;