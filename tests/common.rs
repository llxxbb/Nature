extern crate nature;
extern crate nature_db;
extern crate rocket;

use rocket::local::Client;

use nature::rpc::rocket_server;

#[allow(dead_code)]
pub fn get_test_client() -> Client {
    let rocket = rocket_server();
    Client::new(rocket).expect("valid rocket instance")
}

