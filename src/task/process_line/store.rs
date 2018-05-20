use super::*;

/// born an instance which is the beginning of the changes.
pub fn submit_single(instance: Instance) -> Result<UuidBytes> {
    let task = StoreInfo { instance, converter: None };
    let carrier = Delivery::create_carrier(task)?;
    ProcessLine::store(carrier, Root::Business)
}

pub fn do_store(carrier: Carrier<StoreInfo>, root: Root) -> Result<UuidBytes> {
    let mut instance = carrier.data.instance.clone();
    let uuid = InstanceImpl::verify(&mut instance, root)?;
    let result = TableInstance::insert(&instance);
    match result {
        Ok(_) => {
            send_carrier(CHANNEL_ROUTE.sender.lock().unwrap().clone(), carrier);
            Ok(uuid)
        }
        Err(NatureError::DaoDuplicated) => {
            handle_duplicated(carrier, instance)?;
            Ok(uuid)
        }
        Err(err) => Err(err)
    }
}

fn handle_duplicated(carrier: Carrier<StoreInfo>, instance: Instance) -> Result<()> {
    let define = ThingDefineCache::get(&instance.data.thing)?;
    // **None Status Thing** won't try again
    if !define.is_status() {
        Delivery::finish_carrier(&carrier.id)?;
        return Ok(());
    }
    // same source of **Status Thing** can't store more than once.
    if let Ok(true) = TableInstance::is_exists(&instance) {
        Delivery::finish_carrier(&carrier.id)?;
        return Ok(());
    };
    re_dispatch(carrier)
}