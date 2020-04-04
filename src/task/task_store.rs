use std::sync::mpsc::Sender;

use nature_common::{DynamicConverter, Instance, is_false, Result};
use nature_db::{MetaCacheGetter, MetaGetter, Mission, MissionRaw, RawTask, TaskType};

#[derive(Debug, Clone, Default)]
pub struct TaskForStore {
    pub instance: Instance,
    pub next_mission: Vec<Mission>,
    /// when state-instance saved conflict, use this property to redo the task
    pub previous_mission: Option<Mission>,
    /// to avoid save conflict
    pub need_cache: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct TaskForStoreTemp {
    pub instance: Instance,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub next_mission: Vec<MissionRaw>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub previous_mission: Option<MissionRaw>,
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub need_cache: bool,
}

impl From<TaskForStore> for TaskForStoreTemp {
    fn from(input: TaskForStore) -> Self {
        TaskForStoreTemp {
            instance: input.instance,
            next_mission: {
                let mut rtn: Vec<MissionRaw> = vec![];
                input.next_mission.into_iter().for_each(|one| rtn.push(MissionRaw::from(one)));
                rtn
            },
            previous_mission: match input.previous_mission {
                None => None,
                Some(m) => Some(MissionRaw::from(m))
            },
            need_cache: input.need_cache,
        }
    }
}

impl TaskForStore {
    pub fn send(&self, raw: &RawTask, sender: &Sender<(TaskForStore, RawTask)>) {
        let _ = sender.send((self.to_owned(), raw.to_owned()));
    }

    pub fn for_dynamic(instance: &Instance, dynamic: Vec<DynamicConverter>, previous_mission: Option<Mission>, need_cache: bool) -> Result<TaskForStore> {
        let target = Mission::for_dynamic(dynamic)?;
        // save to task to make it can redo
        let task = TaskForStore::new(instance.clone(), target, previous_mission, need_cache);
        Ok(task)
    }

    pub fn new(instance: Instance, next_mission: Vec<Mission>, previous_mission: Option<Mission>, need_cache: bool) -> Self {
        TaskForStore {
            instance,
            next_mission,
            previous_mission,
            need_cache,
        }
    }

    pub fn to_raw(&self) -> Result<RawTask> {
        let temp = TaskForStoreTemp::from(self.clone());
        let json = serde_json::to_string(&temp)?;
        RawTask::from_str(&json, &self.instance.get_key(), TaskType::Store as i8, &self.instance.meta)
    }

    pub fn from_raw(raw: &RawTask, mc_g: MetaCacheGetter, m_g: &MetaGetter) -> Result<Self> {
        let temp: TaskForStoreTemp = serde_json::from_str(&raw.data)?;
        let rtn = TaskForStore {
            instance: temp.instance,
            next_mission: {
                let mut rtn: Vec<Mission> = vec![];
                for one in temp.next_mission {
                    rtn.push(Mission::from(Mission::from_raw(&one, mc_g, m_g)?))
                }
                rtn
            },
            previous_mission: match temp.previous_mission {
                None => None,
                Some(m) => Some(Mission::from(Mission::from_raw(&m, mc_g, m_g)?))
            },
            need_cache: temp.need_cache,
        };
        Ok(rtn)
    }
}