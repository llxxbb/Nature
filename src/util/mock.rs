use std::sync::*;

pub fn change_mode<T>(mode: &Mutex<T>, value: T) {
    let mut guard = mode.lock().unwrap();
    *guard = value;
}

