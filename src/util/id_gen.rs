use global::*;
use serde::Serialize;
use serde_json;
use uuid::*;

pub fn generate_id<T: ?Sized + Serialize>(value: &T) -> Result<u128> {
    let json = serde_json::to_string(value)?;
    let uuid = Uuid::new_v3(&NAMESPACE_DNS, &json);
    Ok(u128::from_bytes(*uuid.as_bytes()))
}