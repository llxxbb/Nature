use std::str::FromStr;

use nature_common::{Instance, NatureError, Result, Meta, ThingType};
use nature_db::{Mission, RawTask, RawThingDefine, TaskType};

use crate::system::CONTEXT_TARGET_INSTANCE_ID;
use crate::task::TaskForStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskForConvert {
    pub from: Instance,
    pub target: Mission,
    pub last_status: Option<Instance>,
}

impl Default for TaskForConvert {
    fn default() -> Self {
        TaskForConvert {
            from: Instance::default(),
            target: Mission::default(),
            last_status: None,
        }
    }
}

impl TaskForConvert {
    pub fn gen_task<FT, FIG>(task: &TaskForStore, meta_getter: FT, instance_getter: FIG) -> Result<Vec<(TaskForConvert, RawTask)>>
        where FT: Fn(&Meta) -> Result<RawThingDefine>, FIG: Fn(u128) -> Result<Option<Instance>>
    {
        let mut new_carriers: Vec<(TaskForConvert, RawTask)> = Vec::new();
        let missions = task.mission.clone().unwrap();
        for c in missions {
            match Self::new_one(&task.instance, &c, &meta_getter, &instance_getter) {
                Err(err) => return Err(err),
                Ok(x) => {
                    let car = RawTask::new(&x, &c.to.get_full_key(), TaskType::Convert as i16)?;
                    new_carriers.push((x, car));
                }
            }
        }
        Ok(new_carriers)
    }

    fn new_one<FT, FIG>(instance: &Instance, mapping: &Mission, meta_getter: &FT, instance_getter: &FIG) -> Result<TaskForConvert>
        where FT: Fn(&Meta) -> Result<RawThingDefine>,
              FIG: Fn(u128) -> Result<Option<Instance>>
    {
        let define = match mapping.to.get_meta_type() {
            ThingType::Dynamic => RawThingDefine::default(),
            _ => meta_getter(&mapping.to)?
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
                demand.check(&last.status)?;
            }
        };
        let rtn = TaskForConvert {
            from: instance.clone(),
            target: mapping.clone(),
            last_status: last_target,
        };
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
//            svc_define: mockers.s_tmeta_cache.clone(),
//        }
//    }
}