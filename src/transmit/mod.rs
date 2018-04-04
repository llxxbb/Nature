use uuid::UuidBytes;

///! Transmit `Thing`s to process

/// A Loader for `Thing`s and tasks
pub struct Capsule<T> {
    _id: UuidBytes,
    _data: T,
    _transmitted_times: u8,
    _create_time: u64,
    _execute_time: u64,
}

pub trait Transmit{
    fn commit();
}

pub struct TransmitService;

impl Transmit for TransmitService{
    fn commit() {
        unimplemented!()
    }
}