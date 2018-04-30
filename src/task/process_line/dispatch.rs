use super::*;

pub fn do_dispatch(carrier: Carrier<RouteInfo>) {
    let id = &carrier.id.clone();
    let new_carriers = generate_carriers(carrier);
    if new_carriers.is_err() {
        return;
    }
    let new_carriers = new_carriers.unwrap();
    if new_carriers.len() == 0 {
        return;
    }
    let to_send = new_carriers.clone();
    // save news
    for n in new_carriers {
        let _ = CarrierDaoService::insert(&n);
    }
    // remove old
    let _ = CarrierDaoService::delete(id);
    // to carry
    for task in to_send {
        send_carrier(CHANNEL_CONVERT.sender.lock().unwrap().clone(), task)
    }
}

pub fn re_dispatch(convert_info: ConverterInfo) -> Result<()> {
    let carrier = Carrier::new(convert_info)?;
    let _ = CarrierDaoService::insert(&carrier);
    send_carrier(CHANNEL_CONVERT.sender.lock().unwrap().clone(), carrier);
    Ok(())
}

fn generate_carriers(carrier: Carrier<RouteInfo>) -> Result<Vec<Carrier<ConverterInfo>>> {
    let mut new_carriers: Vec<Carrier<ConverterInfo>> = Vec::new();
    let maps = carrier.data.maps.clone();
    for c in maps {
        let task = ConverterInfo {
            from: carrier.instance.clone(),
            mapping: c,
        };
        match Carrier::new(task) {
            Ok(x) => new_carriers.push(x),
            Err(err) => {
                ProcessLine::move_to_err(err, carrier);
                return Ok(new_carriers);
            }
        }
    }
    Ok(new_carriers)
}