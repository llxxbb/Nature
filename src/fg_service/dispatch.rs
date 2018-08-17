use global::*;
use std::marker::PhantomData;
use super::*;

pub trait DispatchServiceTrait {
    fn do_dispatch_task(carrier: Carrier<StoreTaskInfo>);
    fn re_dispatch(carrier: Carrier<StoreTaskInfo>) -> Result<()>;
}

pub struct DispatchServiceImpl<T> {
    delivery_service: PhantomData<T>
}

impl<T: DeliveryServiceTrait> DispatchServiceTrait for DispatchServiceImpl<T> {
    fn do_dispatch_task(carrier: Carrier<StoreTaskInfo>) {
        debug!("------------------do_dispatch_task------------------------");
        if carrier.content.data.mission.is_none() {
            let _ = T::finish_carrier(carrier.id);
            return;
        }

        let converters = match Self::generate_converter_info(&carrier) {
            Ok(new) => new,
            Err(err) => match err.err {
                NatureError::DaoEnvironmentError(_) => return,
                _ => {
                    T::move_to_err(err.err, carrier);
                    return;
                }
            }
        };
        let biz = carrier.instance.thing.key.clone();
        let new_carriers = match T::create_batch_and_finish_carrier(converters, carrier, biz, DataType::Dispatch as u8) {
            Ok(ncs) => ncs,
            Err(_) => return,
        };

        for task in new_carriers {
            T::send_carrier(&CHANNEL_CONVERT.sender, task)
        }
    }

    /// Get last status version and re-convert
    fn re_dispatch(carrier: Carrier<StoreTaskInfo>) -> Result<()> {
        if carrier.upstream.is_none() {
            T::move_to_err(NatureError::InstanceStatusVersionConflict, carrier);
            return Err(NatureErrorWrapper::from(NatureError::InstanceStatusVersionConflict));
        }
        let converter = &carrier.content.data.upstream.clone().unwrap();
        let task = ConverterInfo::new(&converter.from, &converter.target)?;
        let carrier = T::create_and_finish_carrier(task, carrier, converter.target.to.key.clone(), DataType::Convert as u8)?;
        T::send_carrier(&CHANNEL_CONVERT.sender, carrier);
        Ok(())
    }
}

impl<T: DeliveryServiceTrait> DispatchServiceImpl<T> {
    fn generate_converter_info(carrier: &Carrier<StoreTaskInfo>) -> Result<Vec<ConverterInfo>> {
        let mut new_carriers: Vec<ConverterInfo> = Vec::new();
        let target = carrier.mission.clone();
        let tar = target.unwrap();
        for c in tar {
            match ConverterInfo::new(&carrier.instance, &c) {
                Err(err) => return Err(err),
                Ok(x) => new_carriers.push(x),
            }
        }
        Ok(new_carriers)
    }
}
