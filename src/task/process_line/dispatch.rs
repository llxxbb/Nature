use std::marker::PhantomData;
use super::*;

pub type DispatchTask = Dispatch<DeliveryService>;

pub trait DispatchTrait {
    fn do_dispatch_task(carrier: Carrier<RouteInfo>);
    fn re_dispatch(carrier: Carrier<StoreInfo>) -> Result<()>;
}

pub struct Dispatch<T> {
    delivery_service: PhantomData<T>
}

impl<T: DeliveryTrait> DispatchTrait for Dispatch<T> {
    fn do_dispatch_task(carrier: Carrier<RouteInfo>) {
        if carrier.content.data.maps.len() == 0 {
            let _ = T::finish_carrier(&carrier.id);
            return;
        }

        let converters = match Self::generate_converter_info(&carrier) {
            Ok(new) => new,
            Err(NatureError::DaoEnvironmentError(_)) => return,
            Err(err) => {
                T::move_to_err(err, carrier);
                return;
            }
        };
        let biz = carrier.instance.thing.key.clone();
        let new_carriers = match T::create_batch_and_finish_carrier(converters, carrier, biz, DataType::Dispatch as u8) {
            Ok(ncs) => ncs,
            Err(_) => return,
        };

        for task in new_carriers {
            send_carrier(CHANNEL_CONVERT.sender.lock().unwrap().clone(), task)
        }
    }

    /// Get last status version and re-convert
    fn re_dispatch(carrier: Carrier<StoreInfo>) -> Result<()> {
        if carrier.converter.is_none() {
            T::move_to_err(NatureError::InstanceStatusVersionConflict, carrier);
            return Err(NatureError::InstanceStatusVersionConflict);
        }
        let converter = &carrier.content.data.converter.clone().unwrap();
        let task = ConverterInfo::new(&converter.from, &converter.mapping)?;
        let carrier = T::create_and_finish_carrier(task, carrier, converter.mapping.to.key.clone(), DataType::Convert as u8)?;
        send_carrier(CHANNEL_CONVERT.sender.lock().unwrap().clone(), carrier);
        Ok(())
    }
}

impl<T: DeliveryTrait> Dispatch<T> {
    fn generate_converter_info(carrier: &Carrier<RouteInfo>) -> Result<Vec<ConverterInfo>> {
        let mut new_carriers: Vec<ConverterInfo> = Vec::new();
        for c in &carrier.content.data.maps {
            match ConverterInfo::new(&carrier.instance, &c) {
                Err(err) => return Err(err),
                Ok(x) => new_carriers.push(x),
            }
        }
        Ok(new_carriers)
    }
}
