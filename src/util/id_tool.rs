use serde::Serialize;
use serde_json;
use uuid::*;
use nature_common::*;

#[inline]
pub fn generate_id<T: ?Sized + Serialize>(value: &T) -> Result<u128> {
    let json = serde_json::to_string(value)?;
    let uuid = Uuid::new_v3(&NAMESPACE_DNS, &json);
    Ok(u128::from_bytes(*uuid.as_bytes()))
}

#[inline]
pub fn u128_to_vec_u8(value: u128) -> Vec<u8> {
    u128::to_bytes(value).to_vec()
}

#[inline]
pub fn vec_to_u128(vec: &Vec<u8>) -> u128 {
    let mut arr = [0u8; 16];
    for i in 0..16 {
        arr[i] = vec[i];
    }
    u128::from_bytes(arr)
}