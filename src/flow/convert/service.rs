use std::collections::HashSet;
use std::iter::Iterator;
use std::rc::Rc;
use std::str::FromStr;

use crate::system::CONTEXT_TARGET_INSTANCE_ID;

use super::*;

pub trait ConvertServiceTrait {
    fn callback(&self, delayed: DelayedInstances) -> Result<()>;
    fn convert(&self, task: &ConverterInfo, carrier: &RawTask);
}

pub struct ConvertServiceImpl {
    pub svc_task: Rc<TaskServiceTrait>,
    pub caller: Rc<CallOutTrait>,
}

impl ConvertServiceTrait for ConvertServiceImpl {
    fn callback(&self, delayed: DelayedInstances) -> Result<()> {
        match TaskDaoImpl::get(&delayed.carrier_id) {
            Ok(raw) => {
                match raw {
                    None => Err(NatureError::VerifyError("task data missed, maybe it had done already.".to_string())),
                    Some(carrier) => match delayed.result {
                        CallbackResult::Err(err) => {
                            let err = NatureError::ConverterLogicalError(err);
                            let _ = TaskDaoImpl::raw_to_error(&err, &carrier);
                            Err(err)
                        }
                        CallbackResult::Instances(mut ins) => {
                            let task: ConverterInfo = serde_json::from_str(&carrier.data)?;
                            self.handle_instances(&task, &carrier, &mut ins)
                        }
                    }
                }
            }
            Err(e) => Err(e)
        }
    }

    fn convert(&self, task: &ConverterInfo, carrier: &RawTask) {
        debug!("------------------do_convert_task------------------------");
        let parameter = Self::gen_out_parameter(task, carrier.task_id.clone());
        match self.caller.convert(&task.target, &parameter) {
            Ok(ConverterReturned::Instances(mut instances)) => {
                debug!("converted {} instances for `Thing`: {:?}", instances.len(), &task.target.to);
                match self.handle_instances(task, &carrier, &mut instances) {
                    Ok(_) => (),
                    Err(err) => match err {
                        NatureError::DaoEnvironmentError(_) => (),
                        _ => {
                            let _ = TaskDaoImpl::raw_to_error(&err, &carrier);
                        }
                    }
                }
            }
            Ok(ConverterReturned::Delay(delay)) => {
                let _ = TaskDaoImpl::update_execute_time(&carrier.task_id, i64::from(delay));
            }
            Ok(ConverterReturned::LogicalError(ss)) => {
                let _ = TaskDaoImpl::raw_to_error(&NatureError::ConverterLogicalError(ss), &carrier);
            }
            Ok(ConverterReturned::EnvError) => (),
            Ok(ConverterReturned::None) => (),
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => {
                    let _ = TaskDaoImpl::raw_to_error(&err, &carrier);
                }
            }
        };
    }
}

impl ConvertServiceImpl {
    pub fn generate<FD, FT, FIG>(store_task: &StoreTaskInfo, raw: &RawTask,
                                 raw_delete: FD, thing_getter: FT, instance_getter: FIG) -> Result<Vec<(ConverterInfo, RawTask)>>
        where FD: Fn(&[u8]) -> Result<usize>,
              FT: Fn(&Thing) -> Result<RawThingDefine>,
              FIG: Fn(u128) -> Result<Option<Instance>>
    {
        debug!("------------------channel_stored------------------------");
        let biz = store_task.instance.thing.get_full_key();
        if store_task.mission.is_none() {
            debug!("no follow data for : {}", biz);
            let _ = raw_delete(&&raw.task_id);
            return Err(NatureError::Break);
        }
        Self::gen_task(&store_task, thing_getter, instance_getter)
    }

    fn gen_task<FT, FIG>(task: &StoreTaskInfo, thing_getter: FT, instance_getter: FIG) -> Result<Vec<(ConverterInfo, RawTask)>>
        where FT: Fn(&Thing) -> Result<RawThingDefine>, FIG: Fn(u128) -> Result<Option<Instance>>
    {
        let mut new_carriers: Vec<(ConverterInfo, RawTask)> = Vec::new();
        let missions = task.mission.clone().unwrap();
        for c in missions {
            match Self::new_one(&task.instance, &c, &thing_getter, &instance_getter) {
                Err(err) => return Err(err),
                Ok(x) => {
                    let car = RawTask::new(&x, &c.to.get_full_key(), TaskType::Convert as i16)?;
                    new_carriers.push((x, car));
                }
            }
        }
        Ok(new_carriers)
    }

    fn new_one<FT, FIG>(instance: &Instance, mapping: &Mission, thing_getter: &FT, instance_getter: &FIG) -> Result<ConverterInfo>
        where FT: Fn(&Thing) -> Result<RawThingDefine>,
              FIG: Fn(u128) -> Result<Option<Instance>>
    {
        let define = match mapping.to.get_thing_type() {
            ThingType::Dynamic => RawThingDefine::default(),
            _ => thing_getter(&mapping.to)?
        };
        let last_target = if define.is_status() {
            match instance.context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
                // context have target id
                Some(status_id) => {
                    let status_id = u128::from_str(status_id)?;
                    match instance_getter(status_id) {
                        Ok(ins) => ins,
                        Err(_) => return Err(NatureError::Break)
                    }
                }
                None => None,
            }
        } else { None };
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

    fn handle_instances(&self, task: &ConverterInfo, carrier: &RawTask, instances: &mut Vec<Instance>) -> Result<()> {
        // check `ThingType` for Null
        if task.target.to.get_thing_type() == ThingType::Null {
            let rtn = Converted {
                done_task: carrier.to_owned(),
                converted: Vec::new(),
            };
            let _ = CHANNEL_CONVERTED.sender.lock().unwrap().send((task.to_owned(), rtn));
            return Ok(());
        }
        // check status version to avoid loop
        let _ = instances.iter_mut().map(|one: &mut Instance| {
            one.data.thing = task.target.to.clone();
            let _ = one.fix_id();
            one
        }).collect::<Vec<_>>();
        let instances = self.verify(&task.target.to, &instances)?;
        let rtn = Converted {
            done_task: carrier.to_owned(),
            converted: instances,
        };
        let _ = CHANNEL_CONVERTED.sender.lock().unwrap().send((task.to_owned(), rtn));
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
    fn gen_out_parameter(task: &ConverterInfo, carrier_id: Vec<u8>) -> CallOutParameter {
        CallOutParameter {
            from: task.from.clone(),
            last_status: task.last_status.clone(),
            carrier_id,
        }
    }

    fn verify(&self, to: &Thing, instances: &[Instance]) -> Result<Vec<Instance>> {
        let mut rtn: Vec<Instance> = Vec::new();
        // only one status instance should return
        let define = match to.get_thing_type() {
            ThingType::Dynamic => RawThingDefine::default(),
            // TODO need be replaced
            _ => ThingDefineCacheImpl::get(to)?
        };
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

        // all biz must same to "to" and set id
        for r in instances {
            let mut instance = r.clone();
            instance.data.thing = to.clone();
            let _ = instance.fix_id();
            rtn.push(instance);
        }

        Ok(rtn)
    }
}

#[cfg(test)]
mod test {
    // TODO
//    use mockers::matchers::ANY;
//
//    use super::*;
//
//    #[test]
//    fn convert_for_null_target() {
//        let mocks = MyMocks::new();
//        let service_impl = init_svc(&mocks);
//        mocks.s.expect(mocks.call_out.convert_call(ANY, ANY)
//            .and_return(Ok(ConverterReturned::None)));
//        let info = ConverterInfo::default();
//        let raw = RawTask::new(&info, "hello", 10).unwrap();
//        service_impl.convert(&info, &raw)
//    }
//
//    fn init_svc(mockers: &MyMocks) -> ConvertServiceImpl {
//        ConvertServiceImpl {
//            svc_task: mockers.s_task.clone(),
//            caller: mockers.call_out.clone(),
//            svc_define: mockers.s_thing_define_cache.clone(),
//        }
//    }
}