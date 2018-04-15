use uuid::*;
use serde_json;
use global::*;
use serde::Serialize;

pub fn generate_id<T:?Sized + Serialize>(value : &T)-> Result<UuidBytes>{
    let json = serde_json::to_string(value)?;
     Ok(*Uuid::new_v3(&NAMESPACE_DNS, &json).as_bytes())
}