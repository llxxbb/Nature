use serde_json;
use uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NatureError {
    // outer input verify errors
    SerializeError(String),
    VerifyError(String),
    ThingNotDefined(String),
    InstanceStatusVersionConflict,
    UuidParseError,

    // out converter errors
    ConverterLogicalError(String),
    ConverterEnvironmentError(String),

    // Business Contract errors
    TargetInstanceNotIncludeStatus(String),
    TargetInstanceContainsExcludeStatus(String),

    // internal errors
    DaoEnvironmentError(String),
    DaoLogicalError(String),
    DaoDuplicated,
    R2D2Error(String),
    SystemError(String),
}

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
