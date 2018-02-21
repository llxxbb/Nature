extern crate rocket_contrib;

/// convert Web Request to native request
use biz::{WorldConnectionInput, WorldConnectionService};
use self::rocket_contrib::Json;

pub fn start_rocket_server<T: WorldConnectionService>(server: &'static T) -> ::rocket::Rocket{
    ::rocket::ignite().mount("/", routes![input])
}

#[post("/input", format = "application/json", data = "<data>")]
fn input(data: Json<WorldConnectionInput>) -> Json<Result<u64, &'static str>> {
    Json(Ok(123))
//    Json(Err("bad thing"))
}


#[cfg(test)]
mod test;