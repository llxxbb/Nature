use diesel::result::*;
use nature_common::*;
use global::*;


impl From<Error> for NatureErrorWrapper {
    // put it aside because can't find diesel's Timeout Error
    fn from(err: Error) -> Self {
        match err {
            Error::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => NatureErrorWrapper {
                    err: NatureError::DaoDuplicated("".to_string()),
                },
                DatabaseErrorKind::__Unknown => NatureErrorWrapper {
                    err: NatureError::DaoEnvironmentError(format!("{:?}", info)),
                },
                _ => NatureErrorWrapper {
                    err: NatureError::DaoLogicalError(format!("{:?}", info)),
                },
            },
            _ => NatureErrorWrapper {
                err: NatureError::DaoLogicalError(err.to_string()),
            },
        }
    }
}
