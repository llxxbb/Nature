extern crate rocket_contrib;

/// convert Web Request to native request
use biz::{WorldConnectionInput, WorldConnectionService};
use self::rocket_contrib::Json;
use service::Service;
use std::sync::atomic::{AtomicUsize, Ordering};
use super::super::rocket::{ignite, Rocket, State};

//pub fn start_rocket_server<T: WorldConnectionService + Sync>(server: &'static T) -> Rocket {
//    ignite()
//        .manage(server)
//        .mount("/", routes![input])
//}

//#[post("/input", format = "application/json", data = "<data>")]
//fn input<'a>(data: Json<WorldConnectionInput>, svc: State<&(WorldConnectionService + Sync)>) -> Json<Result<u64, &'a str>> {
//    Json(svc.input(data.0))
//}

pub fn start_rocket_server() -> Rocket {
    ignite()
        .manage(Service)
        .mount("/", routes![input])
}

#[post("/input", format = "application/json", data = "<data>")]
fn input(data: Json<WorldConnectionInput>, svc: State<Service>) -> Json<Result<u64, String>> {
    let x = svc.input(data.0);
    Json(x)
}



#[cfg(test)]
mod test;