extern crate nature;
extern crate rocket;

use nature::rpc::rocket_server;

use self::rocket::local::Client;

#[allow(dead_code)]
pub fn get_test_client() -> Client {
    let rocket = rocket_server();
    Client::new(rocket).expect("valid rocket instance")
}

