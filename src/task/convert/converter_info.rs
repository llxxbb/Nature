use std::str::FromStr;

use nature_common::{Instance, ParaForQueryByID, Result};
use nature_db::{Mission, RawTask, TaskType};

use crate::system::CONTEXT_TARGET_INSTANCE_ID;
use crate::task::TaskForStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskForConvert {
    pub from: Instance,
    pub target: Mission,
    pub last_state: Option<Instance>,
}

impl Default for TaskForConvert {
    fn default() -> Self {
        TaskForConvert {
            from: Instance::default(),
            target: Mission::default(),
            last_state: None,
        }
    }
}

impl TaskForConvert {
    pub fn gen_task<FIG>(task: &TaskForStore, instance_getter: FIG) -> Result<Vec<(TaskForConvert, RawTask)>>
        where FIG: Fn(&ParaForQueryByID) -> Result<Option<Instance>>
    {
        let mut new_carriers: Vec<(TaskForConvert, RawTask)> = Vec::new();
        let missions = task.mission.clone().unwrap();
        for c in missions {
            let x = Self::new_one_task(&task.instance, &c, &instance_getter)?;
            let car = RawTask::new(&x, &c.to.get_full_key(), TaskType::Convert as i16)?;
            new_carriers.push((x, car));
        }
        Ok(new_carriers)
    }

    fn new_one_task<FIG>(instance: &Instance, mission: &Mission, instance_getter: &FIG) -> Result<TaskForConvert>
        where FIG: Fn(&ParaForQueryByID) -> Result<Option<Instance>>
    {
        let last_target = if mission.to.is_state {
            match instance.context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
                // context have target id
                Some(state_id) => {
                    let state_id = u128::from_str(state_id)?;
                    let old = instance_getter(&ParaForQueryByID { id: state_id, meta: mission.to.get_string() })?;
                    old
                }
                None => None,
            }
        } else { None };
        if let Some(ref last) = last_target {
            if let Some(demand) = &mission.states_demand {
                demand.check(&last.states)?;
            }
        };
        let rtn = TaskForConvert {
            from: instance.clone(),
            target: mission.clone(),
            last_state: last_target,
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