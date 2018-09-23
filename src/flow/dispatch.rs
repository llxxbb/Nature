use flow::delivery::DeliveryServiceTrait;
use flow::store::StoreTaskInfo;
use std::marker::PhantomData;
use super::*;

pub trait DispatchServiceTrait {
    fn do_dispatch_task(carrier: Carrier<StoreTaskInfo>);
    fn re_dispatch(carrier: Carrier<StoreTaskInfo>) -> Result<()>;
}

pub struct DispatchServiceImpl<D, C> {
    delivery_service: PhantomData<D>,
    converter_service: PhantomData<C>,
}

impl<D, C> DispatchServiceTrait for DispatchServiceImpl<D, C>
    where D: DeliveryServiceTrait, C: ConvertServiceTrait
{
    fn do_dispatch_task(carrier: Carrier<StoreTaskInfo>) {
        debug!("------------------do_dispatch_task------------------------");
        if carrier.content.data.mission.is_none() {
            let _ = D::finish_carrier(carrier.id);
            return;
        }
        let converters = match Self::generate_converter_info(&carrier) {
            Ok(new) => new,
            Err(err) => match err {
                NatureError::DaoEnvironmentError(_) => return,
                _ => {
                    D::move_to_err(err, &carrier);
                    return;
                }
            }
        };
        let biz = &carrier.instance.thing.key;
        if let Ok(_) = D::create_batch_and_finish_carrier(&converters, &carrier) {
            debug!("will dispatch {} convert tasks for `Thing` : {:?}", converters.len(), biz);
            for task in converters {
                D::send_carrier(&CHANNEL_CONVERT.sender, task)
            }
        };
    }

    /// Get last status version and re-convert
    fn re_dispatch(carrier: Carrier<StoreTaskInfo>) -> Result<()> {
        if carrier.upstream.is_none() {
            D::move_to_err(NatureError::InstanceStatusVersionConflict, &carrier);
            return Err(NatureError::InstanceStatusVersionConflict);
        }
        let converter = &carrier.content.data.upstream.clone().unwrap();
        let task = C::new(&converter.from, &converter.target)?;
        let carrier = D::create_and_finish_carrier(task, carrier, converter.target.to.key.clone(), DataType::Convert as u8)?;
        D::send_carrier(&CHANNEL_CONVERT.sender, carrier);
        Ok(())
    }
}

impl<D, C> DispatchServiceImpl<D, C>
    where D: DeliveryServiceTrait, C: ConvertServiceTrait
{
    fn generate_converter_info(carrier: &Carrier<StoreTaskInfo>) -> Result<Vec<Carrier<ConverterInfo>>> {
        let mut new_carriers: Vec<Carrier<ConverterInfo>> = Vec::new();
        let target = carrier.mission.clone();
        let tar = target.unwrap();
        for c in tar {
            match C::new(&carrier.instance, &c) {
                Err(err) => return Err(err),
                Ok(x) => {
                    let car = D::new_carrier(x, &c.to.key, DataType::Convert as u8)?;
                    new_carriers.push(car);
                }
            }
        }
        Ok(new_carriers)
    }
}
