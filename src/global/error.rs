use nature_common::NatureError;
use serde_json;
use std;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use uuid;

#[derive(Debug, Serialize, PartialEq)]
pub struct NatureErrorWrapper {
    pub err: NatureError,
}

impl Deref for NatureErrorWrapper {
    type Target = NatureError;
    fn deref(&self) -> &NatureError {
        &self.err
    }
}

impl From<serde_json::error::Error> for NatureErrorWrapper {
    fn from(e: serde_json::error::Error) -> Self {
        NatureErrorWrapper {
            err: NatureError::SerializeError(e.to_string()),
        }
    }
}

impl From<NatureError> for NatureErrorWrapper {
    fn from(e: NatureError) -> Self {
        NatureErrorWrapper {
            err: e,
        }
    }
}

impl From<uuid::ParseError> for NatureErrorWrapper {
    fn from(_e: uuid::ParseError) -> Self {
        NatureErrorWrapper {
            err: NatureError::UuidParseError,
        }
    }
}

impl From<std::num::ParseIntError> for NatureErrorWrapper {
    fn from(_: std::num::ParseIntError) -> Self {
        NatureErrorWrapper {
            err: NatureError::UuidParseError,
        }
    }
}

impl fmt::Display for NatureErrorWrapper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

