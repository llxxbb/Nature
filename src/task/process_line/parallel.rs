use super::*;

pub fn submit_parallel(batch: ParallelBatchInstance) -> Result<()> {
    match Delivery::create_carrier(batch) {
        Ok(carrier) => {
            // to process asynchronous
            send_carrier(CHANNEL_PARALLEL.sender.lock().unwrap().clone(), carrier);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

pub fn do_parallel(carrier: Carrier<ParallelBatchInstance>) {
    let tasks: Vec<StoreInfo> = carrier.data.0.iter().map(|instance| StoreInfo { instance: instance.clone(), converter: None }).collect();
    let new_carriers = Delivery::create_batch_and_finish_carrier(tasks, carrier);
    if let Ok(nc) = new_carriers {
        for c in nc {
            send_carrier(CHANNEL_STORE.sender.lock().unwrap().clone(), c);
        }
    }
}

