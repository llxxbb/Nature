use chrono::prelude::*;
use global::*;
use serde::Serialize;
use super::schema::delivery;
use uuid::*;
use std::fmt::Debug;
use fg_service::Carrier;

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
            id: {
                let uuid = Uuid::new_v3(&NAMESPACE_DNS, &json);
                uuid.as_bytes().to_vec()
            },
            thing: carrier.content.thing.clone(),
            data_type: carrier.content.data_type as i16,
            data: json,
            create_time: NaiveDateTime::from_timestamp(carrier.create_time, 0),
            execute_time: NaiveDateTime::from_timestamp(carrier.execute_time, 0),
            retried_times: 0,
        })
    }
}
