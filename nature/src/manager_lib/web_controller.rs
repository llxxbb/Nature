use std::convert::TryInto;
use std::num::ParseIntError;

use actix_web::{get, HttpResponse, post, web};
use actix_web::web::Json;

use crate::db::{D_M, D_TE, INS_RANGE, InstanceDaoImpl, MetaDao, RawMeta, RawRelation, TaskErrDao};
use crate::domain::*;
use crate::domain::task::TaskCondition;
use crate::manager_lib::meta_service::MetaService;
use crate::manager_lib::relation_service::RelationService;
use crate::manager_lib::task_err_service::TaskErrService;
use crate::util::js_convert::{to_js_option_output, to_js_vec_output};
use crate::util::web_result;
use crate::vo::{InsCondVO, InstanceVO};

/// ----------------------------------- Instance Operation ----------------------------------------
#[post("/instance/byId")]
async fn get_by_id(para: Json<InsCond>) -> HttpResponse {
    debug!("/instance/byId : {:?}", &para.0);
    let x = InstanceDaoImpl::select_by_id(para.0).await;
    web_result(x)
}

#[post("/instance/byIdJS")]
async fn get_by_id_js(para: Json<InsCondVO>) -> HttpResponse {
    debug!("/instance/byIdJS : {:?}", &para.0);
    let cond = para.0.try_into();
    if cond.is_err() { return web_result(cond); }
    let x = InstanceDaoImpl::select_by_id(cond.unwrap()).await;
    let rtn: Result<Option<InstanceVO>> = to_js_option_output(x);
    web_result(rtn)
}

#[post("/instance/downstream")]
async fn get_downstream_instance(from: String) -> HttpResponse {
    debug!("/instance/downstream : {:?}", &from);
    let x = InstanceDaoImpl::select_downstream(&from).await;
    web_result(x)
}

#[post("/instance/downstreamJS")]
async fn get_downstream_instance_js(from: String) -> HttpResponse {
    debug!("/instance/downstreamJS : {:?}", &from);
    let x = InstanceDaoImpl::select_downstream(&from).await;
    let rtn: Result<Vec<InstanceVO>> = to_js_vec_output(x);
    web_result(rtn)
}

/// fuzzy query
#[post("/instance/byKey")]
async fn get_by_key_range(para: Json<InsCond>) -> HttpResponse {
    debug!("/instance/byKey : {:?}", &para.0);
    let x = INS_RANGE.clone().get_by_key_range(&para.0).await;
    web_result(x)
}

/// fuzzy query
#[post("/instance/byKeyJS")]
async fn get_by_key_range_js(para: Json<InsCondVO>) -> HttpResponse {
    debug!("/instance/byKeyJS : {:?}", &para.0);
    let cond = para.0.try_into();
    if cond.is_err() { return web_result(cond); }
    let x = INS_RANGE.clone().get_by_key_range(&cond.unwrap()).await;
    let rtn: Result<Vec<InstanceVO>> = to_js_vec_output(x);
    web_result(rtn)
}

/// ----------------------------------- Meta ------------------------------------------------------

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

/// ----------------------------------- Relation ----------------------------------------------

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

/// ----------------------------------- Failed Tasks  ----------------------------------------------
#[post("/failed")]
async fn failed_from(para: Json<TaskCondition>) -> HttpResponse {
    let rtn = D_TE.get(&para.0).await;
    web_result(rtn)
}

#[post("/failed/num")]
async fn failed_num_from(para: Json<TaskCondition>) -> HttpResponse {
    debug!("/failed/num : {:?}", &para.0);
    let x = D_TE.get_num(&para.0.task_for).await;
    web_result(x)
}

#[post("/failed/delete")]
async fn failed_delete(task_ids: Json<Vec<String>>) -> HttpResponse {
    let has_err = task_ids.0.iter().find(|x| x.parse::<u64>().is_err());
    if has_err.is_some() {
        return web_result::<Result<i32>>(Err(NatureError::VerifyError("input err".to_string())));
    }
    let ids = task_ids.0.join(",");
    let rtn = D_TE.delete(&ids).await;
    web_result(rtn)
}

#[post("/failed/deleteFor")]
async fn failed_delete_for(task_for: String) -> HttpResponse {
    let rtn = D_TE.delete_for(&task_for).await;
    web_result(rtn)
}

/// reset by task id
#[post("/failed/reset")]
async fn failed_reset(task_ids: Json<Vec<String>>) -> HttpResponse {
    let ids: Vec<std::result::Result<u64, ParseIntError>> = task_ids.iter().map(|x| x.parse::<u64>()).collect();
    if ids.iter().find(|x| x.is_err()).is_some() {
        return web_result::<Result<i32>>(Err(NatureError::VerifyError("input err".to_string())));
    }
    let ids: Vec<u64> = ids.into_iter().map(|x| x.unwrap()).collect();
    let rtn = TaskErrService::move_to_task(ids).await;
    web_result(rtn)
}


pub fn manager_config(cfg: &mut web::ServiceConfig) {
    cfg.service(meta_id_great_than)
        .service(meta_add)
        .service(meta_used)
        .service(meta_delete)
        .service(meta_update)
        .service(get_by_id)
        .service(get_by_id_js)
        .service(get_by_key_range)
        .service(get_by_key_range_js)
        .service(get_downstream_instance)
        .service(get_downstream_instance_js)
        .service(relation_id_great_than)
        .service(relation_update)
        .service(failed_from)
        .service(failed_num_from)
        .service(failed_delete)
        .service(failed_delete_for)
        .service(failed_reset)
    ;
}
