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
        match SD::create_carrier(batch, "", DataType::ParallelBatch as u8) {
            Ok(carrier) => {
                // to process asynchronous
                SD::send_carrier(&CHANNEL_PARALLEL.sender, carrier);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn do_parallel_task(carrier: Carrier<ParallelBatchInstance>) {
        let mut tasks: Vec<Carrier<StoreTaskInfo>> = Vec::new();
        for instance in carrier.content.data.0.iter() {
            match SS::generate_store_task(instance.clone()) {
                Ok(task) => {
                    match SD::new_carrier(task, &instance.thing.key, DataType::Store as u8) {
                        Ok(car) => tasks.push(car),
                        Err(e) => {
                            error!("{}", e);
                            return;
                        }
                    }
                }
                // any error will break the process
                _ => return
            }
        }
        if let Ok(_) = SD::create_batch_and_finish_carrier(&tasks, &carrier) {
            for c in tasks {
                SD::send_carrier(&CHANNEL_STORE.sender, c);
            }
        }
    }
}

