use actix_web::{HttpResponse, web};
use actix_web::web::Json;

use nature_common::*;
use nature_db::{DelayedInstances, RawTask};

use crate::task::IncomeController;

/// **Note** This do not receive System `Meta`'s instances
fn input(instance: Json<Instance>) -> HttpResponse {
    let x = IncomeController::input(instance.0);
    HttpResponse::Ok().json(x)
}

/// Instance with route info
fn self_route(instance: Json<SelfRouteInstance>) -> HttpResponse {
    let x = IncomeController::self_route(instance.0);
    HttpResponse::Ok().json(x)
}

fn callback(delayed: Json<DelayedInstances>) -> HttpResponse {
    let x = IncomeController::callback(delayed.0);
    HttpResponse::Ok().json(x)
}

fn batch_for_serial(serial_batch: Json<TaskForSerial>) -> HttpResponse {
    let x = IncomeController::serial(serial_batch.0);
    HttpResponse::Ok().json(x)
}

fn batch_for_parallel(parallel_batch: Json<TaskForParallel>) -> HttpResponse {
    let x = IncomeController::parallel(parallel_batch.0);
    HttpResponse::Ok().json(x)
}

fn redo_task(task: Json<RawTask>) -> HttpResponse {
    let x = IncomeController::redo_task(task.0);
    HttpResponse::Ok().json(x)
}

pub fn web_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/input").route(web::post().to(input)))
        .service(web::resource("/self_route").route(web::post().to(self_route)))
        .service(web::resource("/callback").route(web::post().to(callback)))
        .service(web::resource("/serial_batch").route(web::post().to(batch_for_serial)))
        .service(web::resource("/parallel_batch").route(web::post().to(batch_for_parallel)))
        .service(web::resource("/redo_task").route(web::post().to(redo_task)));
}