#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

#[cfg(feature = "mysql")]
mod mysql {

}

#[cfg(feature = "sqlite")]
mod sqlite {

}

#[cfg(feature = "mysql")]
pub static CONN_STR: &str = "mysql://root@localhost/nature";
#[cfg(feature = "sqlite")]
pub static CONN_STR: &str = "nature.sqlite";