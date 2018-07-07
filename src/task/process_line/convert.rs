use rpc::*;
use super::*;

pub trait ConvertTaskTrait {
    fn submit_callback(delayed: DelayedInstances) -> Result<()>;
    fn do_convert_task(carrier: Carrier<ConverterInfo>);
}

pub struct ConvertTaskImpl;

impl ConvertTaskTrait for ConvertTaskImpl {
    fn submit_callback(delayed: DelayedInstances) -> Result<()> {
        let carrier = TableDelivery::get::<ConverterInfo>(delayed.carrier_id)?;
        match delayed.result {
            CallbackResult::Err(err) => {
                let err = NatureError::ConverterLogicalError(err);
                DeliveryImpl::<TableDelivery>::move_to_err(err, carrier);
                Ok(())
            }
            CallbackResult::Instances(ins) => handle_instances(&carrier, &ins)
        }
    }
    fn do_convert_task(carrier: Carrier<ConverterInfo>) {
        let para = CallOutParameter::new(&carrier);
        let _ = match ConvertImpl::convert(para) {
            Ok(ConverterReturned::Instances(instances)) => {
                match handle_instances(&carrier, &instances) {
                    Ok(_) => (),
                    Err(NatureError::DaoEnvironmentError(_)) => (),
                    Err(err) => {
                        DeliveryImpl::<TableDelivery>::move_to_err(err, carrier.clone());
                    }
                }
            }
            Ok(ConverterReturned::Delay(delay)) => {
                let _ = TableDelivery::update_execute_time(carrier.id, carrier.execute_time + delay as i64);
                ()
            }
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => DeliveryImpl::<TableDelivery>::move_to_err(err, carrier)
            }
        };
    }
}

fn handle_instances(carrier: &Carrier<ConverterInfo>, instances: &Vec<Instance>) -> Result<()> {
// check status version to avoid loop
    let instances = verify(&carrier.mapping.to, &instances)?;
    let plan = StorePlan::new(&carrier.content.data, &instances)?;
    to_store(carrier, plan);
    Ok(())
}

fn verify(to: &Thing, instances: &Vec<Instance>) -> Result<Vec<Instance>> {
    let mut rtn: Vec<Instance> = Vec::new();

    // only one status instance should return
    let define = ThingDefineCacheImpl::get(to)?;
    if define.is_status() {
        if instances.len() > 1 {
            return Err(NatureError::ConverterLogicalError("[status thing] must return less 2 instances!".to_string()));
        }

        // status version must equal old + 1
        if instances.len() == 1 {
            let mut ins = instances[0].clone();
            ins.data.status_version += 1;
            ins.data.thing = to.clone();
            rtn.push(ins);
        }
        return Ok(rtn);
    }

    // all biz must same to "to"
    for mut r in instances {
        let mut instance = r.clone();
        instance.data.thing = to.clone();
        rtn.push(instance);
    }

    Ok(rtn)
}

fn to_store(carrier: &Carrier<ConverterInfo>, plan: StorePlan) {
    let store_infos: Vec<StoreInfo> = plan.plan.iter().map(|instance| {
        StoreInfo {
            instance: instance.clone(),
            converter: Some(carrier.content.data.clone()),
        }
    }).collect();
    let new_tasks = DeliveryImpl::<TableDelivery>::create_batch_and_finish_carrier(
        store_infos,
        carrier.to_owned(),
        carrier.mapping.to.key.clone(),
        DataType::Convert as u8,
    );
    if new_tasks.is_err() {
        return;
    }
    for task in new_tasks.unwrap() {
        send_carrier(&CHANNEL_STORE.sender, task)
    }
}

pub trait CallOutTrait {
    fn convert(para: CallOutParameter) -> Result<ConverterReturned>;
}