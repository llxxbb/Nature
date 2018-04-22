extern crate rocket_contrib;

use ::rocket::{ignite, Rocket};
use self::rocket_contrib::Json;
use uuid::UuidBytes;
use super::*;


pub fn start_rocket_server() -> Rocket {
    ignite()
        .mount("/", routes![input])
        .mount("/serial_batch", routes![batch_for_serial])
        .mount("/parallel_batch", routes![batch_for_parallel])
}

/// **Note** This do not receive System `Thing`'s instances
#[post("/input", format = "application/json", data = "<instance>")]
fn input(instance: Json<Instance>) -> Json<Result<UuidBytes>> {
    let x = ProcessLine::single_input(instance.0, Root::Business);
    Json(x)
}

#[post("/serial_batch", format = "application/json", data = "<serial_batch>")]
fn batch_for_serial(serial_batch: Json<SerialBatchInstance>) -> Json<Result<()>> {
    let x = InstanceImpl::serial(serial_batch.0);
    Json(x)
}

#[post("/parallel_batch", format = "application/json", data = "<parallel_batch>")]
fn batch_for_parallel(parallel_batch: Json<ParallelBatchInstance>) -> Json<Result<()>> {
    let x = InstanceImpl::parallel(parallel_batch.0);
    Json(x)
}