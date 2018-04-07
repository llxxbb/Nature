use super::*;

pub struct InstanceImpl;

impl InstanceTrait for InstanceImpl {
    fn born(_instance: Instance) -> Result<[u8; 16]> {
        Ok([11, 172, 237, 228, 64, 20, 63, 157, 183, 32, 23, 63, 104, 161, 201, 51])
    }

    fn store(_instance: Instance) -> Result<()> {
        unimplemented!()
    }
}