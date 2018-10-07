use flow::store::StoreServiceTrait;
use std::rc::Rc;
use super::*;

pub trait ParallelServiceTrait {
    fn parallel(&self, batch: ParallelBatchInstance) -> Result<()>;
    fn do_parallel_task(&self, instances: &Vec<Instance>, carrier: RawDelivery);
}

pub struct ParallelServiceImpl {
    delivery_svc: Rc<DeliveryServiceTrait>,
    delivery_dao: Rc<DeliveryDaoTrait>,
    store: Rc<StoreServiceTrait>,
}

impl ParallelServiceTrait for ParallelServiceImpl {
    fn parallel(&self, batch: ParallelBatchInstance) -> Result<()> {
        let raw = RawDelivery::new(&batch, &batch.thing.key, DataType::ParallelBatch as i16)?;
        match self.delivery_dao.insert(&raw) {
            Ok(carrier) => {
                // to process asynchronous
                CHANNEL_PARALLEL.sender.lock().unwrap().send((batch, raw));
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn do_parallel_task(&self, instances: &Vec<Instance>, carrier: RawDelivery) {
        let mut tasks: Vec<RawDelivery> = Vec::new();
        let mut tuple: Vec<(StoreTaskInfo, RawDelivery)> = Vec::new();
        for instance in instances.iter() {
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
                CHANNEL_STORE.sender.lock().unwrap().send(c);
            }
        }
    }
}

