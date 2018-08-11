use chrono::prelude::*;
use serde::Serialize;
use std::fmt::Debug;
use super::super::schema::delivery;
use util::u128_to_vec_u8;
use data::Carrier;
use nature_common::*;

#[derive(Debug)]
#[derive(Insertable)]
#[table_name = "delivery"]
pub struct Delivery {
    pub id: Vec<u8>,
    pub thing: String,
    pub data_type: i16,
    pub data: String,
    pub create_time: NaiveDateTime,
    pub execute_time: NaiveDateTime,
    pub retried_times: i16,
}

impl Delivery {
    pub fn new<T: Serialize + Send + Debug>(carrier: &Carrier<T>) -> Result<Delivery> {
        let json = serde_json::to_string(&carrier.content)?;
        Ok(Delivery {
            id: u128_to_vec_u8(carrier.id),
            thing: carrier.content.thing.clone(),
            data_type: carrier.content.data_type as i16,
            data: json,
            create_time: NaiveDateTime::from_timestamp(carrier.create_time, 0),
            execute_time: NaiveDateTime::from_timestamp(carrier.execute_time, 0),
            retried_times: 0,
        })
    }
}
