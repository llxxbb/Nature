extern crate rocket_contrib;

use ::rocket::{ignite, Rocket, State};
use define::*;
use instance::Instance;
/// convert Web Request to native request
use self::rocket_contrib::Json;
use service::*;
use uuid::UuidBytes;

pub fn start_rocket_server() -> Rocket {
    ignite()
        .manage(NatureService)
        .mount("/", routes![input])
}

#[post("/input", format = "application/json", data = "<instance>")]
fn input(instance: Json<Instance>, svc: State<NatureService>) -> Json<Result<UuidBytes>> {
    let x = svc.flow(instance.0);
    Json(x)
}


#[cfg(test)]
mod test;