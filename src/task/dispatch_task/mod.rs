#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::dispatch_task_impl::*;

#[cfg(test)]
pub mod mock;
#[cfg(not(test))]
pub mod dispatch_task_impl;