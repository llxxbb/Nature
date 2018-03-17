extern crate rocket_contrib;

/// convert Web Request to native request
use biz::{Nature, ThingInstance};
use self::rocket_contrib::Json;
use super::super::rocket::{ignite, Rocket, State};

type WS = &'static (Nature + Sync);

pub fn start_rocket_server(svc: WS) -> Rocket {
    ignite()
        .manage(svc)
        .mount("/", routes![input])
}

#[post("/input", format = "application/json", data = "<data>")]
fn input(data: Json<ThingInstance>, svc: State<WS>) -> Json<Result<[u8; 16], String>> {
    let x = svc.transform(data.0);
    Json(x)
}


#[cfg(test)]
mod test;