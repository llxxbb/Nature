use super::*;

pub fn do_dispatch(carrier: Carrier<RouteInfo>) {
    if carrier.data.maps.len() == 0 {
        let _ = Delivery::finish_carrier(&carrier.id);
        return;
    }

    let converters = match generate_converter_info(&carrier) {
        Ok(new) => new,
        Err(NatureError::DaoEnvironmentError(_)) => return,
        Err(err) => {
            Delivery::move_to_err(err, carrier);
            return;
        }
    };

    let new_carriers = match Delivery::create_batch_and_finish_carrier(converters, carrier) {
        Ok(ncs) => ncs,
        Err(_) => return,
    };

    for task in new_carriers {
        send_carrier(CHANNEL_CONVERT.sender.lock().unwrap().clone(), task)
    }
}

pub fn re_dispatch(carrier: Carrier<StoreInfo>) -> Result<()> {
    if carrier.converter.is_none() {
        Delivery::move_to_err(NatureError::InstanceStatusVersionConflict, carrier);
        return Err(NatureError::InstanceStatusVersionConflict);
    }
    let converter = &carrier.data.converter.clone().unwrap();
    let task = ConverterInfo::new(&converter.from, &converter.mapping)?;
    let carrier = Delivery::create_and_finish_carrier(task, carrier)?;
    send_carrier(CHANNEL_CONVERT.sender.lock().unwrap().clone(), carrier);
    Ok(())
}

fn generate_converter_info(carrier: &Carrier<RouteInfo>) -> Result<Vec<ConverterInfo>> {
    let mut new_carriers: Vec<ConverterInfo> = Vec::new();
    for c in &carrier.data.maps {
        match ConverterInfo::new(&carrier.instance, &c) {
            Err(err) => return Err(err),
            Ok(x) => new_carriers.push(x),
        }
    }
    Ok(new_carriers)
}