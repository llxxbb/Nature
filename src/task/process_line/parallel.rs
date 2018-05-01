use super::*;

pub fn do_parallel(batch: ParallelBatchInstance) -> Result<()> {
    match Delivery::create_carrier(batch) {
        Ok(carrier) => {
            let id = carrier.id.clone();
            to_store(carrier.data)?;
            Delivery::finish_carrier(&id)
        }
        Err(err) => Err(err),
    }
}

fn to_store(batch: ParallelBatchInstance) -> Result<()> {
    for _instance in batch.0{
//        StoreInfo{
//            instance,
//            converter: None,
//        }
    }
    Ok(())
}