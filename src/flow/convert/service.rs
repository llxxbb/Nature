use std::collections::HashSet;
use std::iter::Iterator;
use std::rc::Rc;
use std::str::FromStr;

use crate::system::*;

use super::*;
use nature_db::converter_cfg::{ConverterInfo, Mission, LastStatusDemand};
use nature_db::task_type::TaskType;

pub trait ConvertServiceTrait {
    fn callback(&self, delayed: DelayedInstances) -> Result<()>;
    fn convert(&self, task: &ConverterInfo, carrier: &RawTask);
    fn new_one_converter_info(&self, instance: &Instance, mapping: &Mission) -> Result<ConverterInfo>;
    fn generate_converter_info(&self, task: &StoreTaskInfo) -> Result<Vec<(ConverterInfo, RawTask)>>;
}

pub struct ConvertServiceImpl {
    pub svc_task: Rc<TaskServiceTrait>,
    pub dao_task: Rc<TaskDaoTrait>,
    pub caller: Rc<CallOutTrait>,
    pub svc_define: Rc<ThingDefineCacheTrait>,
    pub dao_instance: Rc<InstanceDaoTrait>,
    pub svc_instance: Rc<InstanceServiceTrait>,
}

impl ConvertServiceTrait for ConvertServiceImpl {
    fn callback(&self, delayed: DelayedInstances) -> Result<()> {
        match self.dao_task.get(&delayed.carrier_id) {
            Ok(raw) => {
                match raw {
                    None => Err(NatureError::VerifyError("task data missed, maybe it had done already.".to_string())),
                    Some(carrier) => match delayed.result {
                        CallbackResult::Err(err) => {
                            let err = NatureError::ConverterLogicalError(err);
                            let _ = self.dao_task.raw_to_error(&err, &carrier);
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
                            let _ = self.dao_task.raw_to_error(&err, &carrier);
                        }
                    }
                }
            }
            Ok(ConverterReturned::Delay(delay)) => {
                let _ = self.dao_task.update_execute_time(&carrier.task_id, i64::from(delay));
            }
            Ok(ConverterReturned::LogicalError(ss)) => {
                let _ = self.dao_task.raw_to_error(&NatureError::ConverterLogicalError(ss), &carrier);
            }
            Ok(ConverterReturned::EnvError) => (),
            Ok(ConverterReturned::None) => (),
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => {
                    let _ = self.dao_task.raw_to_error(&err, &carrier);
                }
            }
        };
    }

    fn new_one_converter_info(&self, instance: &Instance, mapping: &Mission) -> Result<ConverterInfo> {
        let define = match mapping.to.get_thing_type() {
            ThingType::Dynamic => RawThingDefine::default(),
            _ => self.svc_define.get(&mapping.to)?
        };
        let last_target = if define.is_status() {
            match instance.context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
                // context have target id
                Some(status_id) => {
                    let status_id = u128::from_str(status_id)?;
                    self.dao_instance.get_by_id(status_id)?
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

    fn generate_converter_info(&self, task: &StoreTaskInfo) -> Result<Vec<(ConverterInfo, RawTask)>> {
        let mut new_carriers: Vec<(ConverterInfo, RawTask)> = Vec::new();
        let missions = task.mission.clone().unwrap();
        for c in missions {
            match self.new_one_converter_info(&task.instance, &c) {
                Err(err) => return Err(err),
                Ok(x) => {
                    let car = RawTask::new(&x, &c.to.get_full_key(), TaskType::Convert as i16)?;
                    new_carriers.push((x, car));
                }
            }
        }
        Ok(new_carriers)
    }
}

impl ConvertServiceImpl {
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
            let _ = self.svc_instance.verify(one);
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
            _ => self.svc_define.get(to)?
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
            let _ = self.svc_instance.id_generate_if_not_set(&mut instance);
            rtn.push(instance);
        }

        Ok(rtn)
    }
}

#[cfg(test)]
mod test {
    use mockers::matchers::ANY;

    use crate::test_util::*;

    use super::*;

    #[test]
    fn convert_for_null_target() {
        let mocks = MyMocks::new();
        let service_impl = init_svc(&mocks);
        mocks.s.expect(mocks.call_out.convert_call(ANY, ANY)
            .and_return(Ok(ConverterReturned::None)));
        let info = ConverterInfo::default();
        let raw = RawTask::new(&info, "hello", 10).unwrap();
        service_impl.convert(&info, &raw)
    }

    fn init_svc(mockers: &MyMocks) -> ConvertServiceImpl {
        ConvertServiceImpl {
            svc_task: mockers.s_task.clone(),
            dao_task: mockers.d_task.clone(),
            caller: mockers.call_out.clone(),
            svc_define: mockers.s_thing_define_cache.clone(),
            svc_instance: mockers.s_instance.clone(),
            dao_instance: mockers.d_instance.clone(),
        }
    }
}