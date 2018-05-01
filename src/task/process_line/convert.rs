use super::*;

pub fn do_convert(carrier: Carrier<ConverterInfo>) {
    let para = CallOutParameter::new(&carrier);
    let _ = match convert(para) {
        Ok(ConverterReturned::Instances(instances)) => handle_instances(&carrier, &instances),
        Ok(ConverterReturned::Delay(delay)) => {
            let _ = CarrierDaoService::update_execute_time(carrier.id, carrier.execute_time + delay as i64);
            ()
        }
        Err(err) => match err {
            // only **Environment Error** will be retry
            NatureError::ConverterEnvironmentError(_) => (),
            // other error will drop into error
            _ => ProcessLine::move_to_err(err, carrier)
        }
    };
}

pub fn do_callback(delayed: DelayedInstances) {
    match CarrierDaoService::get::<ConverterInfo>(delayed.carrier_id) {
        Ok(carrier) => {
            match delayed.result {
                CallbackResult::Err(err) => {
                    let err = NatureError::ConverterLogicalError(err);
                    ProcessLine::move_to_err(err, carrier)
                }
                CallbackResult::Instances(ins) => handle_instances(&carrier, &ins)
            }
        }
        _ => ()
    }
}

fn handle_instances(carrier: &Carrier<ConverterInfo>, instances: &Vec<Instance>) {
// check status version to avoid loop
    let instances = match verify(&carrier.mapping.to, &instances) {
        Ok(ins) => ins,
        Err(NatureError::DaoEnvironmentError(_)) => return,
        Err(err) => {
            ProcessLine::move_to_err(err, carrier.clone());
            return;
        }
    };
    match StorePlan::new(&carrier.data, &instances) {
        Ok(plan) => to_store(carrier, plan),
        // if store plan error wait to retry
        _ => (),
    }
}

fn verify(to: &Thing, instances: &Vec<Instance>) -> Result<Vec<Instance>> {
    let mut rtn: Vec<Instance> = Vec::new();

    // only one status instance should return
    let define = ThingDefine::new(to)?;
    if define.is_status() {
        if instances.len() > 1 {
            return Err(NatureError::ConverterLogicalError("[status thing] must return less 2 instances!".to_string()));
        }

        // status version must equal old + 1
        if instances.len() == 1 {
            let mut ins = instances[0].clone();
            ins.data.status_version += 1;
            ins.data.thing = to.clone();
            rtn.push(ins);
        }
        return Ok(rtn);
    }

    // all biz must same to "to"
    for mut r in instances {
        let mut instance = r.clone();
        instance.data.thing = to.clone();
        rtn.push(instance);
    }

    Ok(rtn)
}

fn to_store(carrier: &Carrier<ConverterInfo>, plan: StorePlan) {
    let store_infos: Vec<StoreInfo> = plan.plan.iter().map(|instance| {
        StoreInfo {
            instance:instance.clone(),
            converter: Some(carrier.data.clone()),
        }
    }).collect();
    let new_tasks = create_batch_and_finish_carrier(store_infos, carrier.to_owned());
    if new_tasks.is_err() {
        return;
    }
    for task in new_tasks.unwrap() {
        send_carrier(CHANNEL_STORE.sender.lock().unwrap().clone(), task)
    }
}