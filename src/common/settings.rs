use std::env;

lazy_static! {
    pub static ref SEPARATOR_INS_PARA:String={
        env::var("SEPARATOR_INS_PARA").unwrap_or_else(|_| "/".to_string())
    };
    pub static ref SEPARATOR_INS_KEY:String={
        env::var("SEPARATOR_INS_KEY").unwrap_or_else(|_| "|".to_string())
    };
    pub static ref SEPARATOR_META:String={
        env::var("SEPARATOR_META").unwrap_or_else(|_| ":".to_string())
    };
    pub static ref SEPARATOR_META_KEY:String={
        env::var("SEPARATOR_META_KEY").unwrap_or_else(|_| "/".to_string())
    };
}

/// This is only used for deserialize
pub fn default_para_separator() -> String { SEPARATOR_INS_PARA.to_string() }

/// This is only used for serialize
pub fn is_default_para_separator(sep: &str) -> bool {
    sep.eq(&SEPARATOR_INS_PARA.to_string())
}
