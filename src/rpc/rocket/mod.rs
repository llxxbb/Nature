extern crate rocket_contrib;

use ::rocket::{ignite, Rocket, State};
use define::*;
use instance::Instance;
use self::rocket_contrib::Json;
use service::*;
use uuid::UuidBytes;


pub fn start_rocket_server(svc: &'static Nature) -> Rocket {
    ignite()
        .manage(svc)
        .mount("/", routes![input])
}

#[post("/input", format = "application/json", data = "<instance>")]
fn input(instance: Json<Instance>, svc: State<&Nature>) -> Json<Result<UuidBytes>> {
    let x = svc.flow(instance.0);
    Json(x)
}


#[cfg(test)]
mod test;