use nature_common::NatureError;
use serde_json;
use uuid;


impl From<serde_json::error::Error> for NatureError {
    fn from(e: serde_json::error::Error) -> Self {
        NatureError::SerializeError(e.to_string())
    }
}

impl From<uuid::ParseError> for NatureError {
    fn from(_e: uuid::ParseError) -> Self {
        NatureError::UuidParseError
    }
}

impl From<std::num::ParseIntError> for NatureError {
    fn from(_: std::num::ParseIntError) -> Self {
        NatureError::UuidParseError
    }
}

