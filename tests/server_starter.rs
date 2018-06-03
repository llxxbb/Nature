extern crate nature;
extern crate rocket;

use nature::rpc::start_rocket_server;
use rocket::local::Client;


pub fn get_client() -> Client {
    let rocket = start_rocket_server();
    Client::new(rocket).expect("valid rocket instance")
}