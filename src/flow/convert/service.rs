use std::collections::HashSet;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::str::FromStr;
use super::*;
use system::*;

pub trait ConvertServiceTrait {
    fn callback(delayed: DelayedInstances) -> Result<()>;
    fn convert(carrier: Carrier<ConverterInfo>);
    fn new(instance: &Instance, mapping: &Mission) -> Result<ConverterInfo>;
    fn generate_converter_info(carrier: &Carrier<StoreTaskInfo>) -> Result<Vec<Carrier<ConverterInfo>>>;
}

pub struct ConvertServiceImpl<SD, SC, SI> {
    delivery: PhantomData<SD>,
    caller: PhantomData<SC>,
    ins_verify: PhantomData<SI>,
}

impl<SD, SC, SI> ConvertServiceTrait for ConvertServiceImpl<SD, SC, SI>
    where SD: DeliveryServiceTrait,
          SC: CallOutTrait, SI: InstanceServiceTrait {
    fn callback(delayed: DelayedInstances) -> Result<()> {
        let carrier = SD::get::<ConverterInfo>(delayed.carrier_id)?;
        match delayed.result {
            CallbackResult::Err(err) => {
                let err = NatureError::ConverterLogicalError(err);
                SD::move_to_err(err, &carrier);
                Ok(())
            }
            CallbackResult::Instances(mut ins) => Self::handle_instances(&carrier, &mut ins)
        }
    }

    fn convert(carrier: Carrier<ConverterInfo>) {
        debug!("------------------do_convert_task------------------------");
        let parameter = Self::gen_out_parameter(&carrier);
        let _ = match SC::convert(&carrier, &parameter) {
            Ok(ConverterReturned::Instances(mut instances)) => {
                debug!("converted {} instances for `Thing`: {:?}", instances.len(), &carrier.content.thing);
                match Self::handle_instances(&carrier, &mut instances) {
                    Ok(_) => (),
                    Err(err) => match err {
                        NatureError::DaoEnvironmentError(_) => (),
                        _ => SD::move_to_err(err, &carrier)
                    }
                }
            }
            Ok(ConverterReturned::Delay(delay)) => {
                let _ = SD::update_execute_time(carrier.id, carrier.execute_time + delay as i64);
                ()
            }
            Ok(ConverterReturned::LogicalError(ss)) => {
                SD::move_to_err(NatureError::ConverterLogicalError(ss), &carrier)
            }
            Ok(ConverterReturned::EnvError) => (),
            Ok(ConverterReturned::None) => (),
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => SD::move_to_err(err, &carrier)
            }
        };
    }

    fn new(instance: &Instance, mapping: &Mission) -> Result<ConverterInfo> {
        let define = ThingDefineCacheImpl::get(&mapping.to)?;
        let last_target = match define.is_status() {
            false => None,
            true => {
                match instance.context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
                    // context have target id
                    Some(status_id) => {
                        let status_id = u128::from_str(status_id)?;
                        InstanceDaoImpl::get_by_id(status_id)?
                    }
                    None => None,
                }
            }
        };
        if let Some(ref last) = last_target {
            if let Some(demand) = &mapping.last_status_demand {
                Self::check_last(&last.status, demand)?;
            }
        };
        let rtn = ConverterInfo {
            from: instance.clone(),
            target: mapping.clone(),
            last_status: last_target,
        };
        Ok(rtn)
    }

    fn generate_converter_info(carrier: &Carrier<StoreTaskInfo>) -> Result<Vec<Carrier<ConverterInfo>>> {
        let mut new_carriers: Vec<Carrier<ConverterInfo>> = Vec::new();
        let target = carrier.mission.clone();
        let tar = target.unwrap();
        for c in tar {
            match Self::new(&carrier.instance, &c) {
                Err(err) => return Err(err),
                Ok(x) => {
                    let car = SD::new_carrier(x, &c.to.key, DataType::Convert as u8)?;
                    new_carriers.push(car);
                }
            }
        }
        Ok(new_carriers)
    }
}

impl<SD, SC, SI> ConvertServiceImpl<SD, SC, SI>
    where SD: DeliveryServiceTrait,
          SC: CallOutTrait, SI: InstanceServiceTrait {
    fn handle_instances(carrier: &Carrier<ConverterInfo>, instances: &mut Vec<Instance>) -> Result<()> {
        // check status version to avoid loop
        let _ = instances.iter_mut().map(|one: &mut Instance| {
            one.data.thing = carrier.target.to.clone();
            let _ = SI::verify(one);
            one
        }).collect::<Vec<_>>();
        let instances = verify(&carrier.target.to, &instances)?;
        let rtn = Converted {
            done_task: carrier.to_owned(),
            converted: instances,
        };
        let _ = CHANNEL_CONVERTED.sender.lock().unwrap().send(rtn);
        Ok(())
    }

    fn check_last(last: &HashSet<String>, demand: &LastStatusDemand) -> Result<()> {
        for s in &demand.target_status_include {
            if !last.contains(s) {
                return Err(NatureError::TargetInstanceNotIncludeStatus(s.clone()));
            }
        }
        for s in &demand.target_status_include {
            if last.contains(s) {
                return Err(NatureError::TargetInstanceContainsExcludeStatus(s.clone()));
            }
        }
        Ok(())
    }
    fn gen_out_parameter(internal: &Carrier<ConverterInfo>) -> CallOutParameter {
        CallOutParameter {
            from: internal.from.clone(),
            last_status: internal.last_status.clone(),
            carrier_id: internal.id.clone(),
        }
    }
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

