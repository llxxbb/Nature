use std::env;
use std::thread;
use std::time::Duration;

use futures::executor::block_on;

use nature::system::sys_init;

#[allow(dead_code)]
//pub static CONN_STR : &str = "mysql://root@localhost/nature";
pub static CONN_STR: &str = "nature.sqlite";

pub fn test_init() {
    env::set_var("DATABASE_URL", CONN_STR);
    thread::spawn(|| block_on(sys_init()));
}

pub fn sleep(how_long: u64) {
    thread::sleep(Duration::from_millis(how_long));
}
