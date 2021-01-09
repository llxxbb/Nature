use actix_web::{get, HttpResponse, Responder, web};

#[get("/metaRange/{from}")]
async fn meta_range(web::Path(from): web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("get from: {}", from))
}

pub fn manager_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(meta_range);
}
