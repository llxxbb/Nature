use serde::Serialize;
use super::*;

#[derive(Debug)]
pub struct CarryError<T> where T: Sized + Serialize {
    pub err: NatureError,
    pub carrier: Carrier<T>,
}
