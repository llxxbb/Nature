use std::env;

lazy_static! {
    pub static ref SEPARATOR_INS_PARA:String={
        let rtn = env::var("SEPARATOR_INS_PARA").unwrap_or_else(|_| "/".to_string());
        info!("SEPARATOR_INS_PARA: {}", rtn);
        rtn
    };
    pub static ref SEPARATOR_INS_KEY:String={
        let rtn = env::var("SEPARATOR_INS_KEY").unwrap_or_else(|_| "|".to_string());
        info!("SEPARATOR_INS_KEY: {}", rtn);
        rtn
    };
    pub static ref SEPARATOR_TASK_KEY:String={
        let rtn = env::var("SEPARATOR_TASK_KEY").unwrap_or_else(|_| "|".to_string());
        info!("SEPARATOR_TASK_KEY: {}", rtn);
        rtn
    };
    pub static ref SEPARATOR_META:String={
        let rtn = env::var("SEPARATOR_META").unwrap_or_else(|_| ":".to_string());
        info!("SEPARATOR_META: {}", rtn);
        rtn
    };
    pub static ref SEPARATOR_META_KEY:String={
        let rtn = env::var("SEPARATOR_META_KEY").unwrap_or_else(|_| "/".to_string());
        info!("SEPARATOR_META_KEY: {}", rtn);
        rtn
    };
    pub static ref SWITCH_SAVE_DIRECTLY_FOR_ONE : bool = {
        let rtn = env::var("SWITCH_SAVE_DIRECTLY_FOR_ONE").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap();
        info!("SWITCH_SAVE_DIRECTLY_FOR_ONE: {}", rtn);
        rtn
    };
    pub static ref CACHE_SAVED_TIME : u64 = {
        let rtn = env::var("CACHE_SAVED_TIME").unwrap_or_else(|_| "90".to_string()).parse::<u64>().unwrap();
        info!("CACHE_SAVED_TIME: {}", rtn);
        rtn
    };
    pub static ref INSTANCE_CONTENT_MAX_LENGTH : usize = {
        let rtn = env::var("INSTANCE_CONTENT_MAX_LENGTH").unwrap_or_else(|_| "65535".to_string()).parse::<usize>().unwrap();
        info!("INSTANCE_CONTENT_MAX_LENGTH: {}", rtn);
        rtn
    };
    pub static ref INSTANCE_CONTEXT_MAX_LENGTH : usize = {
        let rtn = env::var("INSTANCE_CONTEXT_MAX_LENGTH").unwrap_or_else(|_| "65535".to_string()).parse::<usize>().unwrap();
        info!("INSTANCE_CONTEXT_MAX_LENGTH: {}", rtn);
        rtn
    };
    pub static ref TASK_CONTENT_MAX_LENGTH : usize = {
        let rtn = env::var("TASK_CONTENT_MAX_LENGTH").unwrap_or_else(|_| "16777215".to_string()).parse::<usize>().unwrap();
        info!("TASK_CONTENT_MAX_LENGTH: {}", rtn);
        rtn
    };

    pub static ref QUERY_SIZE_LIMIT : i32 = {
        let rtn = env::var("QUERY_SIZE_LIMIT").unwrap_or_else(|_| "1000".to_string()).parse::<i32>().unwrap();
        info!("QUERY_SIZE_LIMIT: {}", rtn);
        rtn
    };

    pub static ref PLUGIN_PATH : String = {
        let rtn = env::var("PLUGIN_PATH").unwrap_or_else(|_| "plugin/".to_string());
        info!("PLUGIN_PATH: {}", rtn);
        rtn
    };
}

pub fn show_config() {
    info!("nature settings -------------------------");
    let _ = SEPARATOR_INS_PARA.to_string();
    let _ = SEPARATOR_INS_KEY.to_string();
    let _ = SEPARATOR_TASK_KEY.to_string();
    let _ = SEPARATOR_META.to_string();
    let _ = SEPARATOR_META_KEY.to_string();
    let _ = SWITCH_SAVE_DIRECTLY_FOR_ONE.to_string();
    let _ = CACHE_SAVED_TIME.to_string();
    let _ = INSTANCE_CONTENT_MAX_LENGTH.to_string();
    let _ = INSTANCE_CONTEXT_MAX_LENGTH.to_string();
    let _ = TASK_CONTENT_MAX_LENGTH.to_string();
    let _ = QUERY_SIZE_LIMIT.to_string();
    let _ = PLUGIN_PATH.to_string();
    info!("nature settings -------------------------");
}

/// This is only used for deserialize
pub fn default_para_separator() -> String { SEPARATOR_INS_PARA.to_string() }

/// This is only used for serialize
pub fn is_default_para_separator(sep: &str) -> bool {
    sep.eq(&SEPARATOR_INS_PARA.to_string())
}
