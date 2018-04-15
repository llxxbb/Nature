extern crate r2d2;

use chrono::prelude::*;
use data::thing::*;
use global::*;
#[cfg(not(test))]
pub use self::instance_impl::*;
#[cfg(test)]
pub use self::mock::*;
use util::*;
use uuid::*;

/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Instance {
    /// A unique value used to distinguish other instance
    pub id: UuidBytes,
    pub data: InstanceNoID,
}

impl Instance {
    pub fn new_batch_for_serial(batch: &mut SerialBatchInstance) -> Result<Instance> {
        // veriry all
        for mut instance in &mut batch.instance {
            InstanceImpl::verify(&mut instance)?;
        }
        let instance = Instance {
            id: {
                // id based on instance list in `SerialBatchInstance`
                let vec = batch.instance.iter().map(|x| &x.id).collect::<Vec<_>>();
                generate_id(&vec)?
            },
            data: InstanceNoID {
                thing: Thing {
                    key: SYS_KEY_BATCH_SERIAL.to_string(),
                    version: 1,
                },
                execute_time: Local::now().timestamp_millis(),
                create_time: Local::now().timestamp_millis(),
                content: String::new(),
                context: String::new(),
            },
        };
        Ok(instance)
    }
}


/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InstanceNoID {
    /// This instance's Type
    pub thing: Thing,
    /// The time which plan to flow for this instance
    pub execute_time: i64,
    /// When this instance created
    pub create_time: i64,
    /// What contend in this instance for the `Thing`
    pub content: String,
    /// Is a json for a `Map[key, value]` which contents other instance for other `Thing`'s.
    /// `Nature` can transform those to `Instance`'s by flowing.
    ///
    /// # Key
    ///
    /// context name
    ///
    /// # Value
    ///
    /// json data for a `Instance`.
    pub context: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParallelBatchInstance(Vec<Instance>);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SerialBatchInstance {
    pub context_for_finish: String,
    pub ignore_error: bool,
    pub instance: Vec<Instance>,
}

pub trait InstanceTrait {
    /// born an instance which is the beginning of the changes.
    fn born(instance: Instance) -> Result<UuidBytes>;
    fn serial(batch: SerialBatchInstance) -> Result<()>;
    fn parallel(batch: ParallelBatchInstance) -> Result<()>;
}


pub mod instance_impl;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
pub mod test;