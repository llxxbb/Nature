use chrono::prelude::*;
use serde::Serialize;
use super::*;
use super::schema::delivery;

#[derive(Debug)]
#[derive(Insertable)]
#[table_name = "delivery"]
pub struct Delivery<'a> {
    pub id: Vec<u8>,
    pub thing: &'a str,
    pub data_type: i16,
    pub data: &'a str,
    pub create_time: &'a NaiveDateTime,
    pub execute_time: &'a NaiveDateTime,
    pub retried_times: i16,
}

impl<'a> Delivery<'a> {
    pub fn new<T: Serialize + Send>(carrier: &'a Carrier<T>) -> Delivery {
        Delivery {
            id: carrier,
            thing: "",
            data_type: 0,
            data: "",
            create_time: (),
            execute_time: (),
            retried_times: 0,
        }
    }
}
