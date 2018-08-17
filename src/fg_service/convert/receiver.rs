use data::*;
use db::*;
use fg_service::*;
use global::*;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::str::FromStr;

pub trait ConvertServiceTrait {
    fn submit_callback(delayed: DelayedInstances) -> Result<()>;
    fn do_convert_task(carrier: Carrier<ConverterInfo>);
}

pub struct ConvertServiceImpl<SP, SD, SS, SC> {
    plan: PhantomData<SP>,
    delivery: PhantomData<SD>,
    store: PhantomData<SS>,
    caller: PhantomData<SC>,
}

impl<SP, SD, SS, SC> ConvertServiceTrait for ConvertServiceImpl<SP, SD, SS, SC>
    where SP: PlanServiceTrait, SD: DeliveryServiceTrait, SS: StoreServiceTrait, SC: CallOutTrait {
    fn submit_callback(delayed: DelayedInstances) -> Result<()> {
        let carrier = SD::get::<ConverterInfo>(delayed.carrier_id)?;
        match delayed.result {
            CallbackResult::Err(err) => {
                let err = NatureError::ConverterLogicalError(err);
                SD::move_to_err(err, carrier);
                Ok(())
            }
            CallbackResult::Instances(ins) => Self::handle_instances(&carrier, &ins)
        }
    }
    fn do_convert_task(carrier: Carrier<ConverterInfo>) {
        let _ = match SC::convert(&carrier) {
            Ok(ConverterReturned::Instances(instances)) => {
                match Self::handle_instances(&carrier, &instances) {
                    Ok(_) => (),
                    Err(err) => match err.err {
                        NatureError::DaoEnvironmentError(_) => (),
                        _ => SD::move_to_err(err.err, carrier.clone())
                    }
                }
            }
            Ok(ConverterReturned::Delay(delay)) => {
                let _ = SD::update_execute_time(carrier.id, carrier.execute_time + delay as i64);
                ()
            }
            Ok(ConverterReturned::LogicalError(ss)) => {
                SD::move_to_err(NatureError::ConverterLogicalError(ss), carrier.clone())
            }
            Ok(ConverterReturned::EnvError) => (),
            Err(err) => match err.err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => SD::move_to_err(err.err, carrier)
            }
        };
    }
}

impl<SP, SD, SS, SC> ConvertServiceImpl<SP, SD, SS, SC>
    where SP: PlanServiceTrait, SD: DeliveryServiceTrait, SS: StoreServiceTrait, SC: CallOutTrait {
    fn handle_instances(carrier: &Carrier<ConverterInfo>, instances: &Vec<Instance>) -> Result<()> {
// check status version to avoid loop
        let instances = verify(&carrier.target.to, &instances)?;
        let plan = SP::new(&carrier.content.data, &instances)?;
        Self::do_store(carrier, plan);
        Ok(())
    }
    fn do_store(carrier: &Carrier<ConverterInfo>, plan: PlanInfo) {
        let mut store_infos: Vec<StoreTaskInfo> = Vec::new();
        for instance in plan.plan.iter() {
            match SS::generate_store_task(instance.clone()) {
                Ok(task) => store_infos.push(task),
                // break process will environment error occurs.
                _ => return
            }
        }
        let new_tasks = SD::create_batch_and_finish_carrier(
            store_infos,
            carrier.to_owned(),
            carrier.target.to.key.clone(),
            DataType::Convert as u8,
        );
        if new_tasks.is_err() {
            return;
        }
        for task in new_tasks.unwrap() {
            SD::send_carrier(&CHANNEL_STORE.sender, task)
        }
    }
}


fn verify(to: &Thing, instances: &Vec<Instance>) -> Result<Vec<Instance>> {
    let mut rtn: Vec<Instance> = Vec::new();

    // only one status instance should return
    let define = ThingDefineCacheImpl::get(to)?;
    if define.is_status() {
        if instances.len() > 1 {
            return Err(NatureErrorWrapper::from(NatureError::ConverterLogicalError("[status thing] must return less 2 instances!".to_string())));
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

impl ConverterInfo {
    /// **Error:**
    /// * Dao
    /// * DefineNotFind
    /// * uuid parse
    pub fn new(instance: &Instance, mapping: &Mission) -> Result<ConverterInfo> {
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

    fn check_last(last: &HashSet<String>, demand: &LastStatusDemand) -> Result<()> {
        for s in &demand.target_status_include {
            if !last.contains(s) {
                return Err(NatureErrorWrapper::from(NatureError::TargetInstanceNotIncludeStatus(s.clone())));
            }
        }
        for s in &demand.target_status_include {
            if last.contains(s) {
                return Err(NatureErrorWrapper::from(NatureError::TargetInstanceContainsExcludeStatus(s.clone())));
            }
        }
        Ok(())
    }

    pub fn gen_out_parameter(internal: &Carrier<ConverterInfo>) -> CallOutParameter {
        CallOutParameter {
            from: internal.from.clone(),
            last_status: internal.last_status.clone(),
            carrier_id: internal.id.clone(),
        }
    }
}

