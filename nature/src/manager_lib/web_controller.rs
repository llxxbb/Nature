use actix_web::{get, HttpResponse, post, web};
use actix_web::web::Json;

use crate::db::{D_M, INS_RANGE, InstanceDaoImpl, MetaDao, RawMeta, RawRelation};
use crate::domain::*;
use crate::manager_lib::meta_service::MetaService;
use crate::manager_lib::relation_service::RelationService;
use crate::util::web_result;

/// ----------------------------------- Instance
#[post("/instance/byId")]
async fn get_by_id(para: Json<KeyCondition>) -> HttpResponse {
    debug!("/instance/byId : {:?}", &para.0);
    let x = InstanceDaoImpl::get_by_id(para.0).await;
    web_result(x)
}

#[post("/instance/downstream")]
async fn get_downstream_instance(from: String) -> HttpResponse {
    debug!("/instance/downstream : {:?}", &from);
    let x = InstanceDaoImpl::get_downstream(&from).await;
    web_result(x)
}

/// fuzzy query
#[post("/instance/byKey")]
async fn get_by_key_range(para: Json<KeyCondition>) -> HttpResponse {
    debug!("/instance/byKey : {:?}", &para.0);
    let x = INS_RANGE.clone().get_by_key_range(&para.0).await;
    web_result(x)
}

/// ----------------------------------- Meta

/// batch query the metas, `from` is index of `id`, ascending order
#[get("/metaIdGreatThan/{from}/{limit}")]
async fn meta_id_great_than(web::Path((from, limit)): web::Path<(i32, i32)>) -> HttpResponse {
    let range = MetaService::id_great_than(from, limit).await;
    web_result(range)
}

/// add one meta
#[post("/meta/add")]
async fn meta_add(web::Path(raw): web::Path<RawMeta>) -> HttpResponse {
    let rtn = D_M.insert(&raw).await;
    web_result(rtn)
}

#[get("/meta/delete/{name}")]
async fn meta_delete(web::Path(name): web::Path<String>) -> HttpResponse {
    let meta = Meta::from_string(&name);
    if meta.is_err() {
        return web_result::<String>(Err(meta.err().unwrap()));
    }
    let rtn = D_M.delete(&meta.unwrap()).await;
    web_result(rtn)
}

#[post("/meta/update")]
async fn meta_update(raw: Json<RawMeta>) -> HttpResponse {
    let rtn = D_M.edit(&raw).await;
    web_result(rtn)
}

/// check meta whether used
#[get("/metaUsed/{name}")]
async fn meta_used(web::Path(name): web::Path<String>) -> HttpResponse {
    // TODO
    HttpResponse::Ok().body(format!("get from: {}", name))
}

/// ----------------------------------- Meta

/// batch query the relations, `from` is index of `id`, ascending order
#[get("/relationIdGreatThan/{from}/{limit}")]
async fn relation_id_great_than(web::Path((from, limit)): web::Path<(i32, i32)>) -> HttpResponse {
    let range = RelationService::id_great_than(from, limit).await;
    web_result(range)
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
        .service(meta_used)
        .service(meta_delete)
        .service(meta_update)
        .service(get_by_id)
        .service(get_by_key_range)
        .service(get_downstream_instance)
        .service(relation_update);
}
