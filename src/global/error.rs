use serde_json;
use uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    // internal errors
    DaoEnvironmentError(String),
    DaoDuplicated,
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