use super::*;

pub struct Teller;

impl Teller {
    pub fn single_input(_instance: Instance) -> Result<UuidBytes> {
        Ok([11, 172, 237, 228, 64, 20, 63, 157, 183, 32, 23, 63, 104, 161, 201, 51])
    }

    pub fn route(_carrier: Carrier<StoreInfo>) {}

    pub fn callback(_delayed: DelayedInstances) -> Result<()>{
        // TODO
        Ok(())
    }
    pub fn parallel(_batch: ParallelBatchInstance) -> Result<()> { Ok(()) }
    pub fn serial(_batch: SerialBatchInstance) -> Result<()> { Ok(()) }
}

