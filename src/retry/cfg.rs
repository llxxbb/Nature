use std::env;

lazy_static! {
    pub static ref MAX_RETRY_TIMES : usize = {
        env::var("MAX_RETRY_TIMES").unwrap_or_else(|_|"6".to_string()).parse::<usize>().unwrap()
    };
    pub static ref FIRST_RETRY_INTERVAL : usize = {
        env::var("FIRST_RETRY_INTERVAL").unwrap_or_else(|_|"5".to_string()).parse::<usize>().unwrap()
    };
    pub static ref MAX_SLEEP : u64 = {
        env::var("MAX_SLEEP").unwrap_or_else(|_|"60000".to_string()).parse::<u64>().unwrap()
    };
    pub static ref MIN_SLEEP : u64 = {
        env::var("MIN_SLEEP").unwrap_or_else(|_|"1".to_string()).parse::<u64>().unwrap()
    };
    pub static ref BUSY_SLEEP : u64 = {
        env::var("BUSY_SLEEP").unwrap_or_else(|_|"2".to_string()).parse::<u64>().unwrap()
    };
    pub static ref NATURE_SERVER_ADDRESS : String = {
        env::var("NATURE_SERVER_ADDRESS").unwrap_or_else(|_|"http://localhost:8080/redo_task".to_string())
    };
}
