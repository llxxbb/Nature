use diesel::result::*;
pub use self::conn::*;
pub use self::models::*;
pub use self::dao::*;

use super::*;

pub mod schema;

mod conn;
mod models;

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

#[cfg(test)]
mod test;

mod dao;