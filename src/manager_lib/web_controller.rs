use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::web::Json;

use crate::db::{RawMeta, RawRelation};

/// batch query the metas
#[get("/metaRange/{from}")]
async fn meta_range(web::Path(from): web::Path<u32>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", from))
}

/// add one meta
#[get("/metaAdd/{name}")]
async fn meta_add(web::Path(name): web::Path<String>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", name))
}

/// add one meta and one relation to it
#[get("/metaAddWithRelation/{name}/{from}")]
async fn meta_add_with_relation(web::Path((_name, from)): web::Path<(String, String)>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", from))
}

/// move mata to another meta
#[get("/metaMove/{from}/{to}")]
async fn meta_move(web::Path((from, _to)): web::Path<(String, String)>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", from))
}

/// check meta whether used
#[get("/metaUsed/{name}")]
async fn meta_used(web::Path(name): web::Path<String>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", name))
}

#[get("/metaDelete/{name}")]
async fn meta_delete(web::Path(name): web::Path<String>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", name))
}

#[post("/metaUpdate")]
async fn meta_update(_meta: Json<RawMeta>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", ""))
}

#[post("/relationUpdate")]
async fn relation_update(_relation: Json<RawRelation>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", "from"))
}


pub fn manager_config(cfg: &mut web::ServiceConfig) {
    cfg.service(meta_range)
        .service(meta_add)
        .service(meta_add_with_relation)
        .service(meta_move)
        .service(meta_used)
        .service(meta_delete)
        .service(meta_update)
        .service(relation_update);
}
