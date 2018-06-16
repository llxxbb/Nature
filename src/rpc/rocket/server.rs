extern crate rocket_contrib;

use ::rocket::{ignite, Rocket};
use self::rocket_contrib::Json;
use std::ops::Deref;
use super::*;

pub fn start_rocket_server() -> Rocket {
    ignite()
        .mount("/", routes![input])
        .mount("/", routes![batch_for_serial])
        .mount("/", routes![batch_for_parallel])
        .mount("/", routes![callback])
}

/// **Note** This do not receive System `Thing`'s instances
#[post("/input", format = "application/json", data = "<instance>")]
fn input(instance: Json<Instance>) -> Json<Result<u128>> {
    let x = StoreTaskImpl::submit_single(
        TASK_DELIVERY.clone().deref(),
        DATA_INSTANCE.clone().deref(),
        DAO_INSTANCE.clone().deref(),
        CACHE_THING_DEFINE.clone().deref(),
        instance.0);
    Json(x)
}

#[post("/callback", format = "application/json", data = "<delayed>")]
fn callback(delayed: Json<DelayedInstances>) -> Json<Result<()>> {
    let x = ConvertTaskImpl::submit_callback(delayed.0);
    Json(x)
}


#[post("/serial_batch", format = "application/json", data = "<serial_batch>")]
fn batch_for_serial(serial_batch: Json<SerialBatchInstance>) -> Json<Result<()>> {
    let x = submit_serial(serial_batch.0);
    Json(x)
}

#[post("/parallel_batch", format = "application/json", data = "<parallel_batch>")]
fn batch_for_parallel(parallel_batch: Json<ParallelBatchInstance>) -> Json<Result<()>> {
    let x = submit_parallel(parallel_batch.0);
    Json(x)
}