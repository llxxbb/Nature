use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub enum NatureError {
    SerializeError(String),
    VerifyError(String),
    ThingNotDefined(String),
    CarrierDaoError(String),
}

impl From<serde_json::error::Error> for NatureError{
    fn from(e: serde_json::error::Error) -> Self {
        NatureError::SerializeError(e.to_string())
    }
}