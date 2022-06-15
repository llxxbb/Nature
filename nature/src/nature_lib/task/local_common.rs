extern crate libloading as lib;

use crate::domain::*;
use std::panic::{catch_unwind, RefUnwindSafe};
use std::sync::Mutex;
use std::time::Duration;

use lru_time_cache::LruCache;

type CALLER<'a, T, R> = lib::Symbol<'a, fn(&T) -> R>;
type LIB = Option<lib::Library>;

lazy_static! {
    static ref CACHE_ENTRY: Mutex<LruCache<String,Option<LibraryEntry>>> = Mutex::new(LruCache::<String, Option<LibraryEntry>>::with_expiry_duration(Duration::from_secs(3600)));
    static ref CACHE_LIB: Mutex<LruCache<String,LIB>> = Mutex::new(LruCache::<String, LIB>::with_expiry_duration(Duration::from_secs(3600)));
}

#[derive(Clone)]
struct LibraryEntry {
    path: String,
    entry: String,
}

pub async fn local_execute<T: RefUnwindSafe, R>(executor: &str, para: &T) -> Result<R> {
    match get_lib_entry(executor) {
        None => return Err(NatureError::VerifyError("can't find the lib entry".to_string())),
        Some(entry) => {
            // get config of lib
            let lib_cache = CACHE_LIB.lock();
            if lib_cache.is_err() {
                let msg = format!("can't get lock for executor: {}", executor);
                return Err(NatureError::EnvironmentError(msg));
            }
            let mut lib_cache = lib_cache.unwrap();
            let path = entry.path.clone();
            // debug!("load library for :[{}]", path);
            let cfg_lib = lib_cache.entry(path.clone()).or_insert_with(move || unsafe {
                match lib::Library::new(path.clone()) {
                    Err(e) => {
                        warn!("load local lib error for path {}, error : {}", path, e);
                        None
                    }
                    Ok(local_lib) => Some(local_lib)
                }
            });
            // get entry to call
            match cfg_lib {
                None => return Err(NatureError::VerifyError("load local lib error for path".to_string())),
                Some(local_lib) => {
                    let fun: CALLER<T, R> = unsafe {
                        local_lib.get(entry.entry.as_bytes())?
                    };
                    match catch_unwind(|| { fun(&para) }) {
                        Ok(rtn) => {
                            Ok(rtn)
                        }
                        Err(e) => {
                            warn!("{:?} return error: {:?}", &entry.entry, e);
                            Err(NatureError::LogicalError("executor implement error".to_string()))
                        }
                    }
                }
            }
        }
    }
}

fn get_lib_entry(path: &str) -> Option<LibraryEntry> {
    let mut cache = CACHE_ENTRY.lock().unwrap();
    let rtn = cache.entry(path.to_string()).or_insert_with(|| {
        match entry_from_str(path) {
            Ok(e) => Some(e),
            Err(_) => {
                error!("can't load library for path : {}", path);
                None
            }
        }
    });
    rtn.clone()
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

#[cfg(test)]
mod test {
    use futures::executor::block_on;

    use super::*;

    #[test]
    fn local_test() {
        let para = ConverterParameter {
            from: Default::default(),
            last_state: None,
            task_id: 0,
            master: None,
            cfg: "".to_string(),
        };
        // path error
        let rtn: Result<ConverterReturned> = block_on(local_execute("error_dll:err_path", &para));
        dbg!(&rtn);
        assert_eq!(rtn.is_err(), true);
        // method error
        let rtn: Result<ConverterReturned> = block_on(local_execute("nature_integrate_test_executor:err_path", &para));
        dbg!(&rtn);
        assert_eq!(rtn.is_err(), true);
        // ok
        let rtn: ConverterReturned = block_on(local_execute("nature_integrate_test_executor:rtn_none", &para)).unwrap();
        assert_eq!(rtn, ConverterReturned::None);
        // convert_before
        let rtn: Result<Instance> = block_on(local_execute("nature_integrate_test_executor:convert_before_test", &Instance::default())).unwrap();
        assert_eq!(rtn.is_ok(), true);
        // convert_after
        let rtn: Result<Vec<Instance>> = block_on(local_execute("nature_integrate_test_executor:convert_after_test", &vec![Instance::default()])).unwrap();
        assert_eq!(rtn.is_ok(), true);
    }
}