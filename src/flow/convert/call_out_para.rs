use nature_common::CallOutParameter;

use crate::flow::ConverterInfo;

pub struct CallOutParaSvc;

impl CallOutParaSvc {
    pub fn gen(task: &ConverterInfo, carrier_id: Vec<u8>) -> CallOutParameter {
        CallOutParameter {
            from: task.from.clone(),
            last_status: task.last_status.clone(),
            carrier_id,
        }
    }
}
