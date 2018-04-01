///! World Connection Service provider
extern crate uuid;

use dao::*;
use define::*;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
pub use self::nature::*;
use self::uuid::NAMESPACE_DNS;
use self::uuid::Uuid;
use self::uuid::UuidBytes;
use serde_json;
use serde_json::Error;
use std::rc::Rc;


struct Service {
    thins_dao: Rc<Box<ThingDao<CONN>>>,
    nature: Rc<Box<Nature<ThingDao<CONN>, InstanceDao>>>,
}


pub mod nature;

#[cfg(test)]
mod test;
