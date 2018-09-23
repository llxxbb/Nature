extern crate libloading as lib;

use lru_time_cache::LruCache;
use nature_common::*;
use std::sync::Mutex;
use std::time::Duration;

type CALLER<'a> = lib::Symbol<'a, fn(&CallOutParameter) -> ConverterReturned>;
type LIB = lib::Library;

lazy_static! {
    static ref CACHE_LIB: Mutex<LruCache<String,LIB>> = Mutex::new(LruCache::<String, LIB>::with_expiry_duration(Duration::from_secs(3600)));
    static ref CACHE_ENTRY: Mutex<LruCache<String,Option<LibraryEntry>>> = Mutex::new(LruCache::<String, Option<LibraryEntry>>::with_expiry_duration(Duration::from_secs(3600)));
}


pub trait LocalExecutorTrait {
    fn execute(executor: &str, para: &CallOutParameter) -> ConverterReturned;
}

pub struct LocalExecutorImpl;

impl LocalExecutorTrait for LocalExecutorImpl {
    fn execute(executor: &str, para: &CallOutParameter) -> ConverterReturned {
        match Self::get_entry(executor) {
            None => ConverterReturned::None,
            Some(entry) => {
                // get config of lib
                let mut lib_cache = CACHE_LIB.lock().unwrap();
                let path = entry.path.clone();
//                debug!("load library for :[{}]", path);
                let cfg_lib = lib_cache.entry(entry.path).or_insert_with(move || lib::Library::new(path).unwrap());
                // get entry to call
//                debug!("call entry for :[{}]", entry.entry);
                let fun: CALLER = unsafe {
                    cfg_lib.get(entry.entry.as_bytes()).unwrap()
                };
                fun(para)
            }
        }
    }
}

#[derive(Clone)]
struct LibraryEntry {
    path: String,
    entry: String,
}

impl LocalExecutorImpl {
    fn get_entry(path: &str) -> Option<LibraryEntry> {
        let mut cache = CACHE_ENTRY.lock().unwrap();
        cache.entry(path.to_string()).or_insert_with(|| {
            match Self::entry_from_str(path) {
                Ok(e) => Some(e),
                Err(_) => {
                    // TODO send a error message out
                    None
                }
            }
        }).clone()
    }

    fn entry_from_str(path: &str) -> Result<LibraryEntry> {
        let x: Vec<&str> = path.split(":").collect();
        if x.len() != 2 {
            Err(NatureError::VerifyError(format!("illegal format : [{}]", path)))
        } else {
            Ok(LibraryEntry {
                path: x[0].to_string(),
                entry: x[1].to_string(),
            })
        }
    }
}

