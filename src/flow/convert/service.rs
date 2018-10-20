use std::collections::HashSet;
use std::iter::Iterator;
use std::rc::Rc;
use std::str::FromStr;
use super::*;
use system::*;

pub trait ConvertServiceTrait {
    fn callback(&self, delayed: DelayedInstances) -> Result<()>;
    fn convert(&self, task: &ConverterInfo, carrier: &RawTask);
    fn new(&self, instance: &Instance, mapping: &Mission) -> Result<ConverterInfo>;
    fn generate_converter_info(&self, task: &StoreTaskInfo) -> Result<Vec<(ConverterInfo, RawTask)>>;
}

pub struct ConvertServiceImpl {
    pub svc_delivery: Rc<DeliveryServiceTrait>,
    pub dao_delivery: Rc<DeliveryDaoTrait>,
    pub caller: Rc<CallOutTrait>,
    pub svc_define: Rc<ThingDefineCacheTrait>,
    pub dao_instance: Rc<InstanceDaoTrait>,
    pub svc_instance: Rc<InstanceServiceTrait>,
}

impl ConvertServiceTrait for ConvertServiceImpl {
    fn callback(&self, delayed: DelayedInstances) -> Result<()> {
        match self.dao_delivery.get(&delayed.carrier_id) {
            Ok(raw) => {
                match raw {
                    None => Err(NatureError::VerifyError("Delivery data missed, maybe it had done already.".to_string())),
                    Some(carrier) => match delayed.result {
                        CallbackResult::Err(err) => {
                            let err = NatureError::ConverterLogicalError(err);
                            let _ = self.dao_delivery.raw_to_error(&err, &carrier);
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
                            let _ = self.dao_delivery.raw_to_error(&err, &carrier);
                        }
                    }
                }
            }
            Ok(ConverterReturned::Delay(delay)) => {
                let _ = self.dao_delivery.update_execute_time(&carrier.task_id, delay as i64);
            }
            Ok(ConverterReturned::LogicalError(ss)) => {
                let _ = self.dao_delivery.raw_to_error(&NatureError::ConverterLogicalError(ss), &carrier);
            }
            Ok(ConverterReturned::EnvError) => (),
            Ok(ConverterReturned::None) => (),
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => {
                    let _ = self.dao_delivery.raw_to_error(&err, &carrier);
                }
            }
        };
    }

    fn new(&self, instance: &Instance, mapping: &Mission) -> Result<ConverterInfo> {
        let define = self.svc_define.get(&mapping.to)?;
        let last_target = match define.is_status() {
            false => None,
            true => {
                match instance.context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
                    // context have target id
                    Some(status_id) => {
                        let status_id = u128::from_str(status_id)?;
                        self.dao_instance.get_by_id(status_id)?
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

    fn generate_converter_info(&self, task: &StoreTaskInfo) -> Result<Vec<(ConverterInfo, RawTask)>> {
        let mut new_carriers: Vec<(ConverterInfo, RawTask)> = Vec::new();
        let missions = task.mission.clone().unwrap();
        for c in missions {
            match self.new(&task.instance, &c) {
                Err(err) => return Err(err),
                Ok(x) => {
                    let car = RawTask::new(&x, &c.to.key, DataType::Convert as i16)?;
                    new_carriers.push((x, car));
                }
            }
        }
        Ok(new_carriers)
    }
}

impl ConvertServiceImpl {
    fn handle_instances(&self, task: &ConverterInfo, carrier: &RawTask, instances: &mut Vec<Instance>) -> Result<()> {
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

    fn verify(&self, to: &Thing, instances: &Vec<Instance>) -> Result<Vec<Instance>> {
        let mut rtn: Vec<Instance> = Vec::new();

        // only one status instance should return
        let define = self.svc_define.get(to)?;
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
}

