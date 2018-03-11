extern crate rocket_contrib;

/// convert Web Request to native request
use biz::{WorldConnectionInput, WorldConnectionService};
use self::rocket_contrib::Json;
use super::super::rocket::{ignite, Rocket, State};

type WS= &'static (WorldConnectionService + Send + Sync);

pub fn start_rocket_server(svc: WS) -> Rocket {
    ignite()
        .manage(svc)
        .mount("/", routes![input])
}

#[post("/input", format = "application/json", data = "<data>")]
fn input(data: Json<WorldConnectionInput>, svc: State<WS>) -> Json<Result<u64, String>> {
    let x = svc.input(data.0);
    Json(x)
}


#[cfg(test)]
mod test;