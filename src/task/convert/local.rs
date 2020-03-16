extern crate libloading as lib;

use std::panic::catch_unwind;
use std::sync::Mutex;
use std::time::Duration;

use lru_time_cache::LruCache;

use nature_common::*;

use crate::task::ExecutorTrait;

type CALLER<'a> = lib::Symbol<'a, fn(&ConverterParameter) -> ConverterReturned>;
type LIB = Option<lib::Library>;

lazy_static! {
    static ref CACHE_LIB: Mutex<LruCache<String,LIB>> = Mutex::new(LruCache::<String, LIB>::with_expiry_duration(Duration::from_secs(3600)));
    static ref CACHE_ENTRY: Mutex<LruCache<String,Option<LibraryEntry>>> = Mutex::new(LruCache::<String, Option<LibraryEntry>>::with_expiry_duration(Duration::from_secs(3600)));
}



pub struct LocalExecutorImpl;

impl ExecutorTrait for LocalExecutorImpl {
    fn execute(&self, executor: &str, para: &ConverterParameter) -> ConverterReturned {
        match Self::get_entry(executor) {
            None => ConverterReturned::None,
            Some(entry) => {
                // get config of lib
                let mut lib_cache = CACHE_LIB.lock().unwrap();
                let path = entry.path.clone();
//                debug!("load library for :[{}]", path);
                let cfg_lib = lib_cache.entry(entry.path).or_insert_with(move || {
                    match lib::Library::new(path.clone()) {
                        Err(e) => {
                            warn!("  load local lib error for path {}, error : {}", path, e);
                            None
                        }
                        Ok(local_lib) => Some(local_lib)
                    }
                });
                // get entry to call
                match cfg_lib {
                    None => ConverterReturned::None,
                    Some(local_lib) => {
                        let fun: CALLER = unsafe {
                            local_lib.get(entry.entry.as_bytes()).unwrap()
                        };
                        match catch_unwind(|| { fun(para) }) {
                            Ok(rtn) => rtn,
                            Err(e) => {
                                warn!("{:?} return error: {:?}", &entry.entry, e);
                                ConverterReturned::LogicalError("executor implement error".to_string())
                            }
                        }
                    }
                }
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
                    error!("can't load library for path : {}", path);
                    None
                }
            }
        }).clone()
    }

    fn entry_from_str(path: &str) -> Result<LibraryEntry> {
        let x: Vec<&str> = path.split(':').collect();
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

