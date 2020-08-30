use std::env;

lazy_static! {
    pub static ref INSTANCE_CONTENT_MAX_LENGTH : usize = {
        env::var("INSTANCE_CONTENT_MAX_LENGTH").unwrap_or_else(|_| "65535".to_string()).parse::<usize>().unwrap()
    };
    pub static ref INSTANCE_CONTEXT_MAX_LENGTH : usize = {
        env::var("INSTANCE_CONTEXT_MAX_LENGTH").unwrap_or_else(|_| "65535".to_string()).parse::<usize>().unwrap()
    };
    pub static ref TASK_CONTENT_MAX_LENGTH : usize = {
        env::var("TASKY_CONTENT_MAX_LENGTH").unwrap_or_else(|_| "16777215".to_string()).parse::<usize>().unwrap()
    };

    pub static ref QUERY_SIZE_LIMIT : i32 = {
        env::var("QUERY_SIZE_LIMIT").unwrap_or_else(|_| "1000".to_string()).parse::<i32>().unwrap()
    };

}
