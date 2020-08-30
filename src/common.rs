pub use callback::*;
pub use converter::*;
pub use error::*;
pub use from_instance::*;
pub use instance::*;
pub use instance_para::*;
pub use loop_context::*;
pub use meta_setting::*;
pub use meta_type::*;
pub use query::*;
pub use settings::*;
pub use state::*;
pub use target_state::*;
pub use util::*;

pub use self::meta::*;

mod callback;
mod converter;
mod error;
mod from_instance;
mod instance;
mod instance_para;
mod loop_context;
mod meta;
mod meta_setting;
mod meta_type;
mod query;
mod settings;
mod state;
mod target_state;
mod util;

pub type Result<T> = std::result::Result<T, NatureError>;

#[cfg(feature = "id64")]
pub type ID = u64;

#[cfg(feature = "id128")]
pub type ID = u128;