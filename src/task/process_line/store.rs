use super::*;

pub fn do_store(carrier: Carrier<StoreInfo>, root: Root) -> Result<UuidBytes> {
    let mut instance = carrier.data.instance.clone();
    let uuid = InstanceImpl::verify(&mut instance, root)?;
    let result = InstanceDaoService::insert(&instance);
    match result {
        Ok(_) => {
            send_carrier(CHANNEL_ROUTE.sender.lock().unwrap().clone(), carrier);
            Ok(uuid)
        }
        Err(NatureError::DaoDuplicated) => handle_duplicated(carrier, instance, uuid),
        Err(err) => Err(err)
    }
}

fn handle_duplicated(carrier: Carrier<StoreInfo>, instance: Instance, uuid: UuidBytes) -> Result<UuidBytes> {
    let define = ThingDefineDaoService::get(&instance.data.thing)?;
    // **None Status Thing** won't try again
    if !define.is_status() {
        CarrierDaoService::delete(&carrier.id)?;
        return Ok(uuid);
    }
    // same source of **Status Thing** can't store more than once.
    if let Ok(true) = InstanceDaoService::source_stored(&instance) {
        CarrierDaoService::delete(&carrier.id)?;
        return Ok(uuid);
    };
    match carrier.data.converter {
        Some(converter) => {
            // need re-converter
            re_dispatch(converter)?;
            CarrierDaoService::delete(&carrier.id)?;
            Ok(uuid)
        }
        None => {
            // return error to direct store outside
            ProcessLine::move_to_err(NatureError::InstanceStatusVersionConflict, carrier);
            Err(NatureError::InstanceStatusVersionConflict)
        }
    }
}