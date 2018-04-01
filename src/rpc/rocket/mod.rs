extern crate rocket_contrib;

/// convert Web Request to native request
use define::*;
use self::rocket_contrib::Json;
use service::NatureService;
use super::super::rocket::{ignite, Rocket, State};
use uuid::UuidBytes;

pub fn start_rocket_server(svc: NatureService) -> Rocket {
    ignite()
        .manage(svc)
        .mount("/", routes![input])
}

#[post("/input", format = "application/json", data = "<instance>")]
pub fn input(instance: Json<Instance>, svc: State<NatureService>) -> Json<Result<UuidBytes>> {
    let x = svc.flow(instance.0);
    Json(x)
}


#[cfg(test)]
mod test;