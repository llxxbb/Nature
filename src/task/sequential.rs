use std::collections::HashMap;
use std::collections::HashSet;

use chrono::prelude::*;
use serde_json;

use nature_common::{BizObject, Instance, MetaType, NatureError, Result, TaskForSerial};

use crate::system::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialFinished {
    pub succeeded_id: Vec<u128>,
    pub errors: Vec<String>,
}

impl SerialFinished {
    pub fn to_virtual_instance(&self, context_for_finish: &str) -> Result<Instance> {
        let json = serde_json::to_string(self)?;
        let mut context: HashMap<String, String> = HashMap::new();
        context.insert(context_for_finish.to_string(), json);
        let time = Local::now().timestamp();
        Ok(Instance {
            id: 0,
            data: BizObject {
                meta: format!("{}{}:1", MetaType::System.get_prefix(), SYS_KEY_SERIAL.clone()),
                content: String::new(),
                context,
                states: HashSet::new(),
                state_version: 0,
                from: None,
                para: String::new(),
            },
            execute_time: time,
            create_time: time,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaskForSerialWrapper;

impl TaskForSerialWrapper {
    pub fn save<FS>(serial: &TaskForSerial, saver: FS) -> Result<SerialFinished>
        where FS: Fn(&Instance) -> Result<usize>
    {
        let mut errors: Vec<String> = Vec::new();
        let mut succeeded_id: Vec<u128> = Vec::new();
        for mut instance in serial.instances.clone() {
            if let Err(err) = instance.revise() {
                errors.push(format!("{:?}", err));
                continue;
            }
            match saver(&instance) {
                Ok(_) => succeeded_id.push(instance.id),
                Err(err) => match err {
                    NatureError::EnvironmentError(_) => return Err(err),
                    NatureError::DaoDuplicated(_) => succeeded_id.push(instance.id),
                    _ => {
                        errors.push(format!("{:?}", err));
                        continue;
                    }
                }
            }
        }
        Ok(SerialFinished { succeeded_id, errors })
    }
}

