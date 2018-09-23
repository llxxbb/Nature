use flow::*;
use flow::convert::caller::CallOutTrait;
use flow::delivery::DeliveryServiceTrait;
use flow::plan::PlanServiceTrait;
use flow::store::StoreServiceTrait;
use flow::store::StoreTaskInfo;
use std::collections::HashSet;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::str::FromStr;
use system::*;

pub trait ConvertServiceTrait {
    fn submit_callback(delayed: DelayedInstances) -> Result<()>;
    fn do_convert_task(carrier: Carrier<ConverterInfo>);
    fn new(instance: &Instance, mapping: &Mission) -> Result<ConverterInfo>;
    fn gen_out_parameter(internal: &Carrier<ConverterInfo>) -> CallOutParameter;
}

pub struct ConvertServiceImpl<SP, SD, SS, SC, SI> {
    plan: PhantomData<SP>,
    delivery: PhantomData<SD>,
    store: PhantomData<SS>,
    caller: PhantomData<SC>,
    ins_verify: PhantomData<SI>,
}

impl<SP, SD, SS, SC, SI> ConvertServiceTrait for ConvertServiceImpl<SP, SD, SS, SC, SI>
    where SP: PlanServiceTrait, SD: DeliveryServiceTrait,
          SS: StoreServiceTrait, SC: CallOutTrait, SI: InstanceServiceTrait {
    fn submit_callback(delayed: DelayedInstances) -> Result<()> {
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

    fn do_convert_task(carrier: Carrier<ConverterInfo>) {
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

    /// **Error:**
/// * Dao
/// * DefineNotFind
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

    fn gen_out_parameter(internal: &Carrier<ConverterInfo>) -> CallOutParameter {
        CallOutParameter {
            from: internal.from.clone(),
            last_status: internal.last_status.clone(),
            carrier_id: internal.id.clone(),
        }
    }
}

impl<SP, SD, SS, SC, SI> ConvertServiceImpl<SP, SD, SS, SC, SI>
    where SP: PlanServiceTrait, SD: DeliveryServiceTrait,
          SS: StoreServiceTrait, SC: CallOutTrait, SI: InstanceServiceTrait {
    fn handle_instances(carrier: &Carrier<ConverterInfo>, instances: &mut Vec<Instance>) -> Result<()> {
        // check status version to avoid loop
        let _ = instances.iter_mut().map(|one: &mut Instance| {
            one.data.thing = carrier.target.to.clone();
            let _ = SI::verify(one);
            one
        }).collect::<Vec<_>>();
        let instances = verify(&carrier.target.to, &instances)?;
        let plan = SP::new(&carrier.content.data, &instances)?;
        Self::do_store(carrier, plan);
        Ok(())
    }
    fn do_store(carrier: &Carrier<ConverterInfo>, plan: PlanInfo) {
        let mut store_infos: Vec<Carrier<StoreTaskInfo>> = Vec::new();
        for instance in plan.plan.iter() {
            match SS::generate_store_task(instance) {
                Ok(task) => {
                    match SD::new_carrier(task, &plan.to.key, DataType::Store as u8) {
                        Ok(x) => store_infos.push(x),
                        Err(e) => {
                            error!("{}", e);
                            SD::move_to_err(e, carrier);
                            return;
                        }
                    }
                }
                // break process will environment error occurs.
                Err(e) => {
                    error!("{}", e);
                    return;
                }
            }
        }
        if let Ok(_) = SD::create_batch_and_finish_carrier(&store_infos, &carrier.to_owned()) {
            for task in store_infos {
                SD::send_carrier(&CHANNEL_STORE.sender, task)
            }
        }
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

