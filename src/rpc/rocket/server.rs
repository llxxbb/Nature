extern crate rocket_contrib;

use ::rocket::{ignite, Rocket};

use nature_common::*;
use nature_db::*;

use crate::task::*;

use self::rocket_contrib::json::Json;

pub fn rocket_server() -> Rocket {
    ignite()
        .mount("/", routes![input])
        .mount("/", routes![batch_for_serial])
        .mount("/", routes![batch_for_parallel])
        .mount("/", routes![callback])
        .mount("/", routes![redo_task])
        .mount("/", routes![self_route])
}

/// **Note** This do not receive System `Thing`'s instances
#[post("/input", format = "application/json", data = "<instance>")]
fn input(instance: Json<Instance>) -> Json<Result<u128>> {
    let x = IncomeController::input(instance.0);
    Json(x)
}

/// Instance with route info
#[post("/self_route", format = "application/json", data = "<instance>")]
fn self_route(instance: Json<SelfRouteInstance>) -> Json<Result<u128>> {
    let x = IncomeController::self_route(instance.0);
    Json(x)
}

#[post("/callback", format = "application/json", data = "<delayed>")]
fn callback(delayed: Json<DelayedInstances>) -> Json<Result<()>> {
    let x = IncomeController::callback(delayed.0);
    Json(x)
}


#[post("/serial_batch", format = "application/json", data = "<serial_batch>")]
fn batch_for_serial(serial_batch: Json<SerialBatchInstance>) -> Json<Result<()>> {
    let x = IncomeController::serial(serial_batch.0);
    Json(x)
}

#[post("/parallel_batch", format = "application/json", data = "<parallel_batch>")]
fn batch_for_parallel(parallel_batch: Json<ParallelBatchInstance>) -> Json<Result<()>> {
    let x = IncomeController::parallel(parallel_batch.0);
    Json(x)
}

#[post("/redo_task", format = "application/json", data = "<task>")]
fn redo_task(task: Json<RawTask>) -> Json<Result<()>> {
    let x = IncomeController::redo_task(task.0);
    Json(x)
}