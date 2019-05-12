use std::collections::HashSet;
use std::rc::Rc;
use std::str::FromStr;

use crate::system::CONTEXT_TARGET_INSTANCE_ID;

use super::*;

pub struct ConvertServiceImpl {
    pub svc_task: Rc<TaskServiceTrait>,
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