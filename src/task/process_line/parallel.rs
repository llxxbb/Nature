use std::marker::PhantomData;
use super::*;

pub trait ParallelServiceTrait {
    fn submit_parallel(batch: ParallelBatchInstance) -> Result<()>;
    fn do_parallel_task(carrier: Carrier<ParallelBatchInstance>);
}

pub struct ParallelServiceImpl<T> {
    phantom: PhantomData<T>
}

impl<T: DeliveryServiceTrait> ParallelServiceTrait for ParallelServiceImpl<T> {
    fn submit_parallel(batch: ParallelBatchInstance) -> Result<()> {
        match T::create_carrier(batch, "".to_string(), DataType::ParallelBatch as u8) {
            Ok(carrier) => {
                // to process asynchronous
                T::send_carrier(&CHANNEL_PARALLEL.sender, carrier);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn do_parallel_task(carrier: Carrier<ParallelBatchInstance>) {
        let tasks: Vec<StoreInfo> = carrier.content.data.0.iter().map(|instance| StoreInfo { instance: instance.clone(), converter: None }).collect();
        let new_carriers = T::create_batch_and_finish_carrier(tasks, carrier, "".to_string(), DataType::ParallelBatch as u8);
        if let Ok(nc) = new_carriers {
            for c in nc {
                T::send_carrier(&CHANNEL_STORE.sender, c);
            }
        }
    }
}

pub type ParallelTask = ParallelServiceImpl<DeliveryService>;
