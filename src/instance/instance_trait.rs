use super::*;

pub trait InstanceTrait {
    /// born an instance which is the beginning of the changes.
    fn born(instance: Instance) -> Result<UuidBytes>;
    fn store(instance: Instance) -> Result<()>;
}
