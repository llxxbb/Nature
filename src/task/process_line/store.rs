use super::*;

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
    if let Ok(true) = TableInstance::source_stored(&instance) {
        Delivery::finish_carrier(&carrier.id)?;
        return Ok(());
    };
    re_dispatch(carrier)
}