use std::convert::TryInto;
use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, post, ResponseError, web};
use actix_web::web::Json;
use crate::common::*;

use crate::db::{INS_RANGE, InstanceDaoImpl, RawTask};
use crate::domain::*;
use crate::nature_lib::dispatcher::IncomeController;
use crate::util::js_convert::{long_to_string, to_js_option_output};
use crate::util::web_result;
use crate::vo::*;

/// **Note** This do not receive System `Meta`'s instances
#[post("/input")]
async fn input(instance: Json<Instance>) -> HttpResponse {
    let x = IncomeController::input(instance.0).await;
    web_result(x)
}

/// **Note** This do not receive System `Meta`'s instances
#[post("/inputJS")]
async fn input_js(instance: Json<InstanceVO>) -> HttpResponse {
    let para = instance.0.try_into();
    if para.is_err() { return web_result(para); }
    let x = IncomeController::input(para.unwrap()).await;
    web_result(long_to_string(x))
}


/// Instance with route info
#[post("/self_route")]
async fn self_route(instance: Json<SelfRouteInsVO>) -> HttpResponse {
    let para = instance.0.try_into();
    if para.is_err() { return web_result(para); }
    let x = IncomeController::self_route(para.unwrap()).await;
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

#[post("/batchJS")]
async fn batch_js(parallel_batch: Json<Vec<InstanceVO>>) -> HttpResponse {
    let para = parallel_batch.0.into_iter().map(|o| { o.try_into() }).collect::<Vec<Result<Instance>>>();
    let find = para.iter().find(|o| { o.is_err() });
    if find.is_some() { return web_result(find.unwrap().clone()); }
    let x = IncomeController::batch(para.into_iter().map(|o| { o.unwrap() }).collect()).await;
    web_result(x)
}

#[post("/task/redo")]
async fn redo_task(task: Json<RawTask>) -> HttpResponse {
    let x = IncomeController::redo_task(task.0).await;
    web_result(x)
}

/// exactly query
#[post("/get/byId")]
async fn get_by_id(para: Json<InsCond>) -> HttpResponse {
    let x = InstanceDaoImpl::select_by_id(para.0).await;
    web_result(x)
}

/// exactly query
#[post("/get/byIdJs")]
async fn get_by_id_js(para: Json<InsCondVO>) -> HttpResponse {
    let cond = para.0.try_into();
    if cond.is_err() { return web_result(cond); }
    let x = InstanceDaoImpl::select_by_id(cond.unwrap()).await;
    let rtn: Result<Option<InstanceVO>> = to_js_option_output(x);
    web_result(rtn)
}

/// fuzzy query
#[post("/get/byKeyRange")]
async fn get_by_key_range(para: Json<InsCondVO>) -> HttpResponse {
    let cond = para.0.try_into();
    if cond.is_err() { return web_result(cond); }
    let x = INS_RANGE.clone().get_by_key_range(&cond.unwrap()).await;
    web_result(x)
}

pub fn web_config(cfg: &mut web::ServiceConfig) {
    cfg.service(input)
        .service(input_js)
        .service(self_route)
        .service(callback)
        .service(batch)
        .service(batch_js)
        .service(redo_task)
        .service(get_by_id)
        .service(get_by_id_js)
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

