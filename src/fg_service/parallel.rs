use global::*;
use std::marker::PhantomData;
use super::*;

pub trait ParallelServiceTrait {
    fn submit_parallel(batch: ParallelBatchInstance) -> Result<()>;
    fn do_parallel_task(carrier: Carrier<ParallelBatchInstance>);
}

pub struct ParallelServiceImpl<SD, SS> {
    delivery: PhantomData<SD>,
    store: PhantomData<SS>,
}

impl<SD, SS> ParallelServiceTrait for ParallelServiceImpl<SD, SS>
    where SD: DeliveryServiceTrait, SS: StoreServiceTrait
{
    fn submit_parallel(batch: ParallelBatchInstance) -> Result<()> {
        match SD::create_carrier(batch, "".to_string(), DataType::ParallelBatch as u8) {
            Ok(carrier) => {
                // to process asynchronous
                SD::send_carrier(&CHANNEL_PARALLEL.sender, carrier);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn do_parallel_task(carrier: Carrier<ParallelBatchInstance>) {
        let mut tasks: Vec<StoreTaskInfo> = Vec::new();
        for instance in carrier.content.data.0.iter() {
            match SS::generate_store_task(instance.clone()) {
                Ok(task) => tasks.push(task),
                // any error will break the process
                _ => return
            }
        }
        let new_carriers = SD::create_batch_and_finish_carrier(tasks, carrier, "".to_string(), DataType::ParallelBatch as u8);
        if let Ok(nc) = new_carriers {
            for c in nc {
                SD::send_carrier(&CHANNEL_STORE.sender, c);
            }
        }
    }
}

