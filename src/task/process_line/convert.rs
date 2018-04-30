use super::*;

pub fn do_convert(carrier: Carrier<ConverterInfo>) {
    let para = match ParaForCallOut::new(&carrier.data) {
        Ok(ci) => ci,
        Err(NatureError::DaoEnvironmentError(_)) => return,
        Err(err) => {
            ProcessLine::move_to_err(err, carrier);
            return;
        }
    };
    let _ = match convert(&para) {
        Ok(instances) => {
            // check status version to avoid loop
            if let Err(err) = verify(&instances) {
                ProcessLine::move_to_err(err, carrier);
                return;
            }
            match StorePlan::new(&carrier.data, &instances) {
                Ok(plan) => to_store(carrier, plan),
                // if store plan error wait to retry
                _ => (),
            }
        }
        Err(err) => match err {
            // only **Environment Error** will be retry
            NatureError::ConverterEnvironmentError(_) => (),
            // other error will drop into error
            _ => ProcessLine::move_to_err(err, carrier)
        }
    };
}

fn verify(_instances: &Vec<Instance>) -> Result<()> {
    // TODO only one status instance should return

    // TODO all biz must same

    // TODO status version must equal old + 1
    Ok(())
}

fn to_store(carrier: Carrier<ConverterInfo>, plan: StorePlan) {
    let mut new_tasks: Vec<Carrier<StoreInfo>> = Vec::new();
    for instance in plan.plan {
        let store_carrier = Carrier::new(StoreInfo {
            instance,
            converter: Some(carrier.data.clone()),
        });
        let _ = match store_carrier {
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