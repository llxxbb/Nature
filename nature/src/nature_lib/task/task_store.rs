use crate::common::Result;
use crate::db::{MetaCache, MetaDao, Mission, MissionRaw, RawTask, TaskType};
use crate::domain::*;
use crate::util::*;

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
    #[serde(skip_serializing_if = "is_default")]
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
        RawTask::from_str(&json, &self.instance.get_key(), TaskType::Store as i8, &self.instance.path.meta)
    }

    pub async fn from_raw<MC, M>(raw: &RawTask, mc_g: &MC, m_g: &M) -> Result<Self>
        where MC: MetaCache, M: MetaDao
    {
        let temp: TaskForStoreTemp = serde_json::from_str(&raw.data)?;
        let rtn = TaskForStore {
            instance: temp.instance,
            next_mission: {
                let mut rtn: Vec<Mission> = vec![];
                for one in temp.next_mission {
                    rtn.push(Mission::from_raw(&one, mc_g, m_g).await?)
                }
                rtn
            },
            previous_mission: match temp.previous_mission {
                None => None,
                Some(m) => Some(Mission::from_raw(&m, mc_g, m_g).await?)
            },
            need_cache: temp.need_cache,
        };
        Ok(rtn)
    }
}