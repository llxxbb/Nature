extern crate chrono;
extern crate fern;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate world_connection;

fn main() {
    // init logger
    world_connection::util::setup_logger().unwrap();

    // read config
    let config = world_connection::util::get_settings();

    info!("##### Server created ---------------------------");
    // create rpc server
    world_connection::rpc::start_web_server(config.get("port").unwrap());
}



