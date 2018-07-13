extern crate nature;
extern crate rocket;

use nature::rpc::start_rocket_server;
use nature::util::setup_logger;
use rocket::local::Client;


pub fn get_client() -> Client {
    let rocket = start_rocket_server();
    let _ = setup_logger();
    Client::new(rocket).expect("valid rocket instance")
}