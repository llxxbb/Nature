extern crate rocket_contrib;

/// convert Web Request to native request
use define::{Nature, ThingInstance};
use self::rocket_contrib::Json;
use super::super::rocket::{ignite, Rocket, State};
use uuid::UuidBytes;

type WS = &'static (Nature + Sync);

pub fn start_rocket_server(svc: WS) -> Rocket {
    ignite()
        .manage(svc)
        .mount("/", routes![input])
}

#[post("/input", format = "application/json", data = "<instance>")]
fn input(instance: Json<ThingInstance>, svc: State<WS>) -> Json<Result<UuidBytes, String>> {
    let x = svc.flow(instance.0);
    Json(x)
}


#[cfg(test)]
mod test;