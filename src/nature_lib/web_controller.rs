use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, post, ResponseError, web};
use actix_web::web::Json;
use serde::export::fmt::Debug;

use crate::db::{InstanceDaoImpl, RawTask};
use crate::domain::*;
use crate::nature_lib::dispatcher::IncomeController;
use crate::nature_lib::web_init::INS_KEY_GT;
use crate::util::web_result;

/// **Note** This do not receive System `Meta`'s instances
#[post("/input")]
async fn input(instance: Json<Instance>) -> HttpResponse {
    let x = IncomeController::input(instance.0).await;
    web_result(x)
}

/// Instance with route info
#[post("/self_route")]
async fn self_route(instance: Json<SelfRouteInstance>) -> HttpResponse {
    let x = IncomeController::self_route(instance.0).await;
    web_result(x)
}

#[post("/callback")]
async fn callback(delayed: Json<DelayedInstances>) -> HttpResponse {
    let x = IncomeController::callback(delayed.0).await;
    web_result(x)
}

#[post("/batch")]
async fn batch(parallel_batch: Json<Vec<Instance>>) -> HttpResponse {
    let x = IncomeController::batch(parallel_batch.0).await;
    web_result(x)
}

#[post("/redo_task")]
async fn redo_task(task: Json<RawTask>) -> HttpResponse {
    let x = IncomeController::redo_task(task.0).await;
    web_result(x)
}

/// exactly query
#[post("/get_by_id")]
async fn get_by_id(para: Json<KeyCondition>) -> HttpResponse {
    let x = InstanceDaoImpl::get_by_id(para.0).await;
    web_result(x)
}

/// fuzzy query
#[post("/get_by_key_range")]
async fn get_by_key_range(para: Json<KeyCondition>) -> HttpResponse {
    let x = INS_KEY_GT.clone().get_by_key_range(&para.0).await;
    web_result(x)
}

pub fn web_config(cfg: &mut web::ServiceConfig) {
    cfg.service(input)
        .service(self_route)
        .service(callback)
        .service(batch)
        .service(redo_task)
        .service(get_by_id)
        .service(get_by_key_range);
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

