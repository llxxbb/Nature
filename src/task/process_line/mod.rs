use dao::*;
pub use self::threads::*;
use serde::Serialize;
use super::*;
use uuid::UuidBytes;

pub struct ProcessLine;

impl ProcessLine {
    /// born an instance which is the beginning of the changes.
    pub fn store(instance: Instance, root: Root) -> Result<UuidBytes> {
        let mut instance = instance;
        let uuid = InstanceImpl::verify(&mut instance, root)?;
        let task = StoreTask(instance);
        let carrier = Carrier::new(task)?;
        let _cid = CarrierDaoService::insert(&carrier)?;
        carrier.take_it_over()?;
        send_carrier(CHANNEL_ROUTE.sender.lock().unwrap().clone(), carrier);
        Ok(uuid)
    }

    fn route(carrier: Carrier<StoreTask>) {
        if let Ok(x) = MappingDaoService::get_relations(&carrier.data.0) {
            if let Some(y) = x {
                let _ = match Carrier::new(y) {
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
            send_carrier(CHANNEL_CONVERTER.sender.lock().unwrap().clone(), task)
        }
    }

    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize {
        let _ = CarrierDaoService::move_to_error(CarryError { err, carrier });
    }
}


mod threads;

#[cfg(test)]
mod test_store;
