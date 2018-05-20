use super::*;

pub fn submit_single(_instance: Instance) -> Result<UuidBytes> {
    Ok([11, 172, 237, 228, 64, 20, 63, 157, 183, 32, 23, 63, 104, 161, 201, 51])
}

pub fn submit_callback(_delayed: DelayedInstances) -> Result<()> {
    Ok(())
}

pub fn submit_parallel(_batch: ParallelBatchInstance) -> Result<()> { Ok(()) }

pub fn submit_serial(_batch: SerialBatchInstance) -> Result<()> { Ok(()) }

