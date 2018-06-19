use std::marker::PhantomData;
use super::*;

pub type ParallelTask = Parallel<DeliveryService>;
pub struct Parallel<T> {
    phantom: PhantomData<T>
}

impl<T: DeliveryTrait> Parallel<T> {
    pub fn submit_parallel(batch: ParallelBatchInstance) -> Result<()> {
        match T::create_carrier(batch, "".to_string(), DataType::ParallelBatch as u8) {
            Ok(carrier) => {
                // to process asynchronous
                send_carrier(CHANNEL_PARALLEL.sender.lock().unwrap().clone(), carrier);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn do_parallel(carrier: Carrier<ParallelBatchInstance>) {
        let tasks: Vec<StoreInfo> = carrier.content.data.0.iter().map(|instance| StoreInfo { instance: instance.clone(), converter: None }).collect();
        let new_carriers = T::create_batch_and_finish_carrier(tasks, carrier, "".to_string(), DataType::ParallelBatch as u8);
        if let Ok(nc) = new_carriers {
            for c in nc {
                send_carrier(CHANNEL_STORE.sender.lock().unwrap().clone(), c);
            }
        }
    }
}
