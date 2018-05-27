use std::sync::*;

pub fn lock_and_set_mock_value<L: Clone, V>(lock: &'static Mutex<L>, mv: &Mutex<V>, rtn: V) -> MutexGuard<'static, L> {
    let tmp_lock = lock.lock().unwrap();
    let mut value = mv.lock().unwrap();
    *value = rtn;
    tmp_lock
}

pub fn set_mock_value<V>(mv: &Mutex<V>, rtn: V)  {
    let mut value = mv.lock().unwrap();
    *value = rtn;
}