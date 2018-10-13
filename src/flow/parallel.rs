use flow::store::StoreServiceTrait;
use std::rc::Rc;
use super::*;

pub trait ParallelServiceTrait {
    fn parallel(&self, batch: ParallelBatchInstance) -> Result<()>;
    fn do_parallel_task(&self, instances: ParallelBatchInstance, carrier: RawDelivery);
}

pub struct ParallelServiceImpl {
    pub delivery_svc: Rc<DeliveryServiceTrait>,
    pub delivery_dao: Rc<DeliveryDaoTrait>,
    pub store: Rc<StoreServiceTrait>,
}

impl ParallelServiceTrait for ParallelServiceImpl {
    fn parallel(&self, batch: ParallelBatchInstance) -> Result<()> {
        let raw = RawDelivery::new(&batch, &batch.thing.key, DataType::ParallelBatch as i16)?;
        match self.delivery_dao.insert(&raw) {
            Ok(_carrier) => {
                // to process asynchronous
                let _ = CHANNEL_PARALLEL.sender.lock().unwrap().send((batch, raw));
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn do_parallel_task(&self, batch: ParallelBatchInstance, carrier: RawDelivery) {
        let mut tasks: Vec<RawDelivery> = Vec::new();
        let mut tuple: Vec<(StoreTaskInfo, RawDelivery)> = Vec::new();
        for instance in batch.instances.iter() {
            match self.store.generate_store_task(instance) {
                Ok(task) => {
                    match RawDelivery::new(&task, &instance.thing.key, DataType::Store as i16) {
                        Ok(car) => {
                            tasks.push(car.clone());
                            tuple.push((task, car))
                        }
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
        if let Ok(_) = self.delivery_svc.create_batch_and_finish_carrier(&tasks, &carrier.id) {
            for c in tuple {
                let _ = CHANNEL_STORE.sender.lock().unwrap().send(c);
            }
        }
    }
}

