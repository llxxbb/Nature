use super::*;

pub fn do_convert(carrier: Carrier<ConverterInfo>) {
    let _ = match convert(&carrier) {
        Ok(instances) => on_success(carrier, instances),
        Err(err) => match err {
            // only **Environment Error** will be retry
            NatureError::ConverterEnvironmentError(_) => (),
            // other error will drop into error
            _ => ProcessLine::move_to_err(err, carrier)
        }
    };
}

fn on_success(carrier: Carrier<ConverterInfo>, instances: Vec<Instance>) {
    // make plan
    let mut plan = StorePlan {
        from_id: carrier.data.0.id,
        to: carrier.data.1.to.clone(),
        plan: instances,
    };
    if let Ok(_) = StorePlanDaoService::save(&mut plan) {
        to_store(carrier, plan);
    };
    // if store plan error wait to retry
}

fn to_store(carrier: Carrier<ConverterInfo>, plan: StorePlan) {
    let mut new_tasks: Vec<Carrier<StoreInfo>> = Vec::new();
    for instance in plan.plan {
        let _ = match Carrier::new(StoreInfo(instance)) {
            Ok(c) => {
                let _ = match CarrierDaoService::insert(&c) {
                    Ok(_) => new_tasks.push(c),
                    Err(_) => return // retry next time
                };
            }
            Err(err) => {
                ProcessLine::move_to_err(err, carrier);
                return;
            }
        };
    }
    if let Ok(_) = CarrierDaoService::delete(&carrier.id) {
        for task in new_tasks {
            send_carrier(CHANNEL_STORE.sender.lock().unwrap().clone(), task)
        }
    };
}