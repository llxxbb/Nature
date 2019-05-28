use std::rc::Rc;

use actix_web::{App, AsyncResponder, Error, http, HttpMessage, HttpRequest, HttpResponse, Json};
use futures::future::Future;

use nature_common::*;
use nature_db::{DelayedInstances, RawTask};

use crate::status::State;
use crate::task::IncomeController;

/// **Note** This do not receive System `Thing`'s instances
fn input(req: &HttpRequest<Rc<State>>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let state = req.state().clone();
    req.json().from_err().and_then(|r: Instance| {
        let x = IncomeController::input(r, state);
        Ok(HttpResponse::Ok().json(x))
    }).responder()
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

pub fn web_app() -> App<Rc<State>> {
    App::with_state(Rc::new(State::new()))
        .resource("/input", |r| r.method(http::Method::POST).f(input))
        .resource("/self_route", |r| r.method(http::Method::POST).with(self_route))
        .resource("/callback", |r| r.method(http::Method::POST).with(callback))
        .resource("/serial_batch", |r| r.method(http::Method::POST).with(batch_for_serial))
        .resource("/parallel_batch", |r| r.method(http::Method::POST).with(batch_for_parallel))
        .resource("/redo_task", |r| r.method(http::Method::POST).with(redo_task))
}