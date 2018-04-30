use super::*;

pub struct ProcessLine;

impl ProcessLine {
    pub fn single_input(_instance: Instance) -> Result<[u8; 16]> {
        Ok([11, 172, 237, 228, 64, 20, 63, 157, 183, 32, 23, 63, 104, 161, 201, 51])
    }

    pub fn route(_carrier: Carrier<StoreInfo>) {}

    pub fn callback(_delayed: DelayedInstances) {
        // TODO
    }
}

