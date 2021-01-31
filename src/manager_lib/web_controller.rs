use actix_web::{get, HttpResponse, post, web};
use actix_web::web::Json;

use crate::db::{INS_RANGE, InstanceDaoImpl, RawMeta, RawRelation};
use crate::domain::*;
use crate::manager_lib::meta_service::MetaService;
use crate::manager_lib::relation_service::RelationService;
use crate::util::web_result;

#[post("/instance/byId")]
async fn get_by_id(para: Json<KeyCondition>) -> HttpResponse {
    debug!("/instance/byId : {:?}", &para.0);
    let x = InstanceDaoImpl::get_by_id(para.0).await;
    web_result(x)
}

/// fuzzy query
#[post("/instance/byKey")]
async fn get_by_key_range(para: Json<KeyCondition>) -> HttpResponse {
    debug!("/instance/byKey : {:?}", &para.0);
    let x = INS_RANGE.clone().get_by_key_range(&para.0).await;
    web_result(x)
}

/// batch query the metas, `from` is index of `id`, ascending order
#[get("/metaIdGreatThan/{from}/{limit}")]
async fn meta_id_great_than(web::Path((from, limit)): web::Path<(i32, i32)>) -> HttpResponse {
    let range = MetaService::id_great_than(from, limit).await;
    web_result(range)
}

/// batch query the relations, `from` is index of `id`, ascending order
#[get("/relationIdGreatThan/{from}/{limit}")]
async fn relation_id_great_than(web::Path((from, limit)): web::Path<(i32, i32)>) -> HttpResponse {
    let range = RelationService::id_great_than(from, limit).await;
    web_result(range)
}

/// add one meta
#[get("/metaAdd/{name}")]
async fn meta_add(web::Path(name): web::Path<String>) -> HttpResponse {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", name))
}

/// add one meta and one relation to it
#[get("/metaAddWithRelation/{name}/{from}")]
async fn meta_add_with_relation(web::Path((_name, from)): web::Path<(String, String)>) -> HttpResponse {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", from))
}

/// move mata to another meta
#[get("/metaMove/{from}/{to}")]
async fn meta_move(web::Path((from, _to)): web::Path<(String, String)>) -> HttpResponse {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", from))
}

/// check meta whether used
#[get("/metaUsed/{name}")]
async fn meta_used(web::Path(name): web::Path<String>) -> HttpResponse {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", name))
}

#[get("/metaDelete/{name}")]
async fn meta_delete(web::Path(name): web::Path<String>) -> HttpResponse {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", name))
}

#[post("/metaUpdate")]
async fn meta_update(_meta: Json<RawMeta>) -> HttpResponse {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", ""))
}

#[post("/relationUpdate")]
async fn relation_update(_relation: Json<RawRelation>) -> HttpResponse {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", "from"))
}


pub fn manager_config(cfg: &mut web::ServiceConfig) {
    cfg.service(meta_id_great_than)
        .service(relation_id_great_than)
        .service(meta_add)
        .service(meta_add_with_relation)
        .service(meta_move)
        .service(meta_used)
        .service(meta_delete)
        .service(meta_update)
        .service(get_by_id)
        .service(get_by_key_range)
        .service(relation_update);
}
