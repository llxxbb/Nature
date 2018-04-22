use dao::*;
use rpc::rocket::*;
pub use self::threads::*;
use serde::Serialize;
use super::*;
use uuid::UuidBytes;

pub struct ProcessLine;

impl ProcessLine {
    /// born an instance which is the beginning of the changes.
    pub fn single_input(instance: Instance) -> Result<UuidBytes> {
        let task = StoreTask(instance);
        let carrier = Carrier::new(task)?;
        let _ = CarrierDaoService::insert(&carrier)?;
        Self::store(carrier, Root::Business)
    }

    fn route(carrier: Carrier<StoreTask>) {
        let instance = &carrier.data.0.clone();
        if let Ok(maps) = MappingDaoService::get_relations(&instance.data.thing) {
            if maps.len() == 0 {
                let _ = CarrierDaoService::delete(&carrier.id);
                return;
            }
            let route = RouteInfo { instance: instance.clone(), maps };
            let _ = match Carrier::new(route) {
                Ok(new_carrier) => {
                    // insert new first carrier
                    if let Ok(_) = CarrierDaoService::insert(&new_carrier) {
                        // then delete old carrier
                        if let Ok(_) = CarrierDaoService::delete(&carrier.id) {
                            send_carrier(CHANNEL_DISPATCH.sender.lock().unwrap().clone(), new_carrier);
                        };
                    };
                }
                Err(err) => Self::move_to_err(err, carrier)
            };
        };
    }

    fn dispatch(carrier: Carrier<RouteInfo>) {
        let mut new_carriers: Vec<Carrier<ConverterTask>> = Vec::new();
        let instance = carrier.instance.clone();
        let maps = carrier.data.maps.clone();
        for c in maps {
            let task = ConverterTask(instance.clone(), c);
            match Carrier::new(task) {
                Ok(x) => new_carriers.push(x),
                Err(err) => {
                    Self::move_to_err(err, carrier);
                    return;
                }
            }
        }
        let to_send = new_carriers.clone();
        // save news
        for n in new_carriers {
            let _ = CarrierDaoService::insert(&n);
        }
        // remove old
        let _ = CarrierDaoService::delete(&carrier.id);
        // do task
        for task in to_send {
            send_carrier(CHANNEL_CONVERT.sender.lock().unwrap().clone(), task)
        }
    }

    fn convert(carrier: Carrier<ConverterTask>) {
        let _ = match convert(&carrier) {
            Ok(instances) => {
                // make plan
                let mut plan = StorePlan {
                    from_id: carrier.data.0.id,
                    to: carrier.data.1.to.clone(),
                    plan: instances,
                };
                if let Ok(_) = StorePlanDaoService::save(&mut plan) {
                    let mut new_tasks: Vec<Carrier<StoreTask>> = Vec::new();
                    for instance in plan.plan {
                        let _ = match Carrier::new(StoreTask(instance)) {
                            Ok(c) => {
                                let _ = match CarrierDaoService::insert(&c) {
                                    Ok(_) => new_tasks.push(c),
                                    Err(_) => return // retry next time
                                };
                            }
                            Err(err) => {
                                Self::move_to_err(err, carrier);
                                return;
                            }
                        };
                    }
                    if let Ok(_) = CarrierDaoService::delete(&carrier.id) {
                        for task in new_tasks {
                            send_carrier(CHANNEL_STORE.sender.lock().unwrap().clone(), task)
                        }
                    };
                };
            }
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => Self::move_to_err(err, carrier)
            }
        };
    }

    fn store(carrier: Carrier<StoreTask>, root: Root) -> Result<UuidBytes> {
        let mut carrier = carrier;
        let uuid = InstanceImpl::verify(&mut carrier.data.0, root)?;
        InstanceDaoService::insert(&carrier.data.0)?;
        send_carrier(CHANNEL_ROUTE.sender.lock().unwrap().clone(), carrier);
        Ok(uuid)
    }

    fn store_for_receive(carrier: Carrier<StoreTask>) {
        let cp = carrier.clone();
        if let Err(err) = Self::store(carrier, Root::Business) {
            Self::move_to_err(err, cp)
        };
    }

    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize {
        let _ = CarrierDaoService::move_to_error(CarryError { err, carrier });
    }
}


mod threads;

#[cfg(test)]
mod test_store;
