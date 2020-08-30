#[cfg(feature = "id128")]
pub use id128::*;
#[cfg(feature = "id64")]
pub use id64::*;

#[cfg(feature = "id128")]
mod id128;
#[cfg(feature = "id64")]
mod id64;
