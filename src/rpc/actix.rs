use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, ResponseError, web};
use actix_web::web::Json;
use serde::export::fmt::Debug;

use nature_common::{Instance, NatureError, ParaForQueryByID, SelfRouteInstance, TaskForParallel, TaskForSerial};
use nature_db::{DelayedInstances, InstanceDaoImpl, RawTask};

use crate::task::IncomeController;

/// **Note** This do not receive System `Meta`'s instances
fn input(instance: Json<Instance>) -> HttpResponse {
    let x = IncomeController::input(instance.0);
    return_result(x)
}

/// Instance with route info
fn self_route(instance: Json<SelfRouteInstance>) -> HttpResponse {
    let x = IncomeController::self_route(instance.0);
    return_result(x)
}

fn callback(delayed: Json<DelayedInstances>) -> HttpResponse {
    let x = IncomeController::callback(delayed.0);
    return_result(x)
}

fn batch_for_serial(serial_batch: Json<TaskForSerial>) -> HttpResponse {
    let x = IncomeController::serial(serial_batch.0);
    return_result(x)
}

fn batch_for_parallel(parallel_batch: Json<TaskForParallel>) -> HttpResponse {
    let x = IncomeController::parallel(parallel_batch.0);
    return_result(x)
}

fn redo_task(task: Json<RawTask>) -> HttpResponse {
    let x = IncomeController::redo_task(task.0);
    return_result(x)
}

fn get_by_id(para: Json<ParaForQueryByID>) -> HttpResponse {
    let x = InstanceDaoImpl::get_by_id(&para.0);
    return_result(x)
}


pub fn web_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/input", web::post().to(input))
        .route("/self_route", web::post().to(self_route))
        .route("/callback", web::post().to(callback))
        .route("/serial_batch", web::post().to(batch_for_serial))
        .route("/parallel_batch", web::post().to(batch_for_parallel))
        .route("/redo_task", web::post().to(redo_task))
        .route("/get_by_id", web::post().to(get_by_id));
}


#[derive(Debug)]
struct WebError {
    err: NatureError,
}

impl Display for WebError {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.err)
    }
}

impl ResponseError for WebError {}

fn return_result<T>(x: nature_common::Result<T>) -> HttpResponse
    where T: serde::Serialize + Debug
{
    HttpResponse::Ok().json(x)
}
