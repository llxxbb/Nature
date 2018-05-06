use super::*;

pub struct Teller;

impl Teller {
    /// born an instance which is the beginning of the changes.
    pub fn single_input(instance: Instance) -> Result<UuidBytes> {
        let task = StoreInfo { instance, converter: None };
        let carrier = Delivery::create_carrier(task)?;
        ProcessLine::store(carrier, Root::Business)
    }

    pub fn callback(delayed: DelayedInstances) -> Result<()> { do_callback(delayed) }

    pub fn parallel(batch: ParallelBatchInstance) -> Result<()> { submit_parallel(batch) }
    pub fn serial(_batch: SerialBatchInstance) -> Result<()> {
        // TODO
        Ok(())
    }
}
