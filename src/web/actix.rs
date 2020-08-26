use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, ResponseError, web};
use actix_web::web::Json;
use serde::export::fmt::Debug;

use nature_common::{DelayedInstances, Instance, KeyCondition, NatureError, SelfRouteInstance};
use nature_db::{InstanceDaoImpl, RawTask};

use crate::controller::IncomeController;
use crate::system::INS_KEY_GT;

/// **Note** This do not receive System `Meta`'s instances
async fn input(instance: Json<Instance>) -> HttpResponse {
    let x = IncomeController::input(instance.0).await;
    return_result(x)
}

/// Instance with route info
async fn self_route(instance: Json<SelfRouteInstance>) -> HttpResponse {
    let x = IncomeController::self_route(instance.0).await;
    return_result(x)
}

async fn callback(delayed: Json<DelayedInstances>) -> HttpResponse {
    let x = IncomeController::callback(delayed.0).await;
    return_result(x)
}

async fn batch(parallel_batch: Json<Vec<Instance>>) -> HttpResponse {
    let x = IncomeController::batch(parallel_batch.0).await;
    return_result(x)
}

async fn redo_task(task: Json<RawTask>) -> HttpResponse {
    let x = IncomeController::redo_task(task.0).await;
    return_result(x)
}

/// exactly query
async fn get_by_id(para: Json<KeyCondition>) -> HttpResponse {
    let x = InstanceDaoImpl::get_by_id(para.0).await;
    return_result(x)
}

/// fuzzy query
async fn get_by_key_range(para: Json<KeyCondition>) -> HttpResponse {
    let x = INS_KEY_GT.clone().get_by_key_range(&para.0).await;
    return_result(x)
}

#[derive(Serialize, Deserialize)]
struct MyStruct {
    name: String
}

// /// fuzzy query
// async fn hello(para: Json<MyStruct>) -> impl Responder {
//     format!("Hello {}!", para.name)
// }
//
//
// #[get("/{id}/{name}/index.html")]
// async fn index(path: Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", path.1, path.0)
// }


pub fn web_config(cfg: &mut web::ServiceConfig) {
    cfg
        // .service(index)
        // .route("/hello", web::post().to(hello))
        .route("/input", web::post().to(input))
        .route("/self_route", web::post().to(self_route))
        .route("/callback", web::post().to(callback))
        .route("/batch", web::post().to(batch))
        .route("/redo_task", web::post().to(redo_task))
        .route("/get_by_id", web::post().to(get_by_id))
        .route("/get_by_key_range", web::post().to(get_by_key_range));
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
