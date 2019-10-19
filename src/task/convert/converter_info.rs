use nature_common::{Instance, Result};
use nature_db::{Mission, RawTask, TaskType};

use crate::task::TaskForStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskForConvert {
    pub from: Instance,
    pub target: Mission,
}

impl Default for TaskForConvert {
    fn default() -> Self {
        TaskForConvert {
            from: Instance::default(),
            target: Mission::default(),
        }
    }
}

impl TaskForConvert {
    pub fn gen_task(task: &TaskForStore) -> Result<Vec<(TaskForConvert, RawTask)>>
    {
        let mut new_carriers: Vec<(TaskForConvert, RawTask)> = Vec::new();
        let missions = task.next_mission.clone().unwrap();
        for c in missions {
            let x = TaskForConvert {
                from: task.instance.clone(),
                target: c.clone(),
            };
            let car = RawTask::new(&x, &c.to.get_full_key(), TaskType::Convert as i16)?;
            new_carriers.push((x, car));
        }
        Ok(new_carriers)
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