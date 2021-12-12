lazy_static! {
    pub static ref REDO_URL : String = {
        env::var("REDO_URL").unwrap_or_else(|_|"http://localhost:8080/task/redo".to_string())
    };
    pub static ref MAX_RETRY_TIMES : usize = {
        env::var("MAX_RETRY_TIMES").unwrap_or_else(|_|"6".to_string()).parse::<usize>().unwrap()
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
    pub static ref BASE_DELAY : i64 = {
        env::var("BASE_DELAY").unwrap_or_else(|_|"5".to_string()).parse::<i64>().unwrap()
    };
    pub static ref LOAD_SIZE : i64 = {
        env::var("LOAD_SIZE").unwrap_or_else(|_|"100".to_string()).parse::<i64>().unwrap()
    };
    pub static ref CLEAN_DELAY : i64 = {
        env::var("CLEAN_DELAY").unwrap_or_else(|_|"2".to_string()).parse::<i64>().unwrap()
    };
}

use std::env;
