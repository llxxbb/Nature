extern crate rocket_contrib;

use ::rocket::{ignite, Rocket};
use define::*;
use instance::*;
use self::rocket_contrib::Json;
use uuid::UuidBytes;

pub fn start_rocket_server() -> Rocket {
    ignite()
        .mount("/", routes![input])
}

#[post("/input", format = "application/json", data = "<instance>")]
fn input(instance: Json<Instance>) -> Json<Result<UuidBytes>> {
    let x = InstanceImpl::born(instance.0);
    Json(x)
}


#[cfg(test)]
mod test;