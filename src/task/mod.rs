use data::*;
use global::*;
pub use self::converter_task::*;
#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::process_line::*;
pub use self::store_task::*;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;

pub trait Task {
    fn take_it_over(&self) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    pub instance: Instance,
    pub maps: Vec<Mapping>,
}

pub mod store_task;

pub mod converter_task;

pub mod process_line;

#[cfg(test)]
mod mock;