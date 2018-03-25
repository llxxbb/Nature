use uuid::UuidBytes;

///! Transmit `Thing`s to process

/// A Loader for `Thing`s and tasks
pub struct Capsule<T> {
    id: UuidBytes,
    data: T,
    transmitted_times: u8,
    create_time: u64,
    execute_time: u64,
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