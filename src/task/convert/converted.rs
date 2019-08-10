use super::*;

pub struct Converted {
    pub done_task: RawTask,
    pub converted: Vec<Instance>,
}

impl Converted {
    pub fn gen<FT>(task: &TaskForConvert, carrier: &RawTask, instances: Vec<Instance>, meta_getter: FT) -> Result<Converted>
        where FT: Fn(&Meta) -> Result<RawThingDefine>
    {
        // check `ThingType` for Null
        if task.target.to.get_meta_type() == ThingType::Null {
            let rtn = Converted {
                done_task: carrier.to_owned(),
                converted: Vec::new(),
            };
            return Ok(rtn);
        }
        // check status version to avoid loop
        let mut fixxed_ins: Vec<Instance> = Vec::new();
        for one in instances {
            let mut n = one.clone();
            n.data.meta = task.target.to.clone();
            let _ = n.fix_id();
            fixxed_ins.push(n)
        }
        let instances = Self::verify(&task.target.to, &fixxed_ins, meta_getter)?;
        let rtn = Converted {
            done_task: carrier.to_owned(),
            converted: instances,
        };
        Ok(rtn)
    }

    fn verify<FT>(to: &Meta, instances: &[Instance], meta_getter: FT) -> Result<Vec<Instance>>
        where FT: Fn(&Meta) -> Result<RawThingDefine>,
    {
        let mut rtn: Vec<Instance> = Vec::new();
        // only one status instance should return
        let define = match to.get_meta_type() {
            ThingType::Dynamic => RawThingDefine::default(),
            // TODO need be replaced
            _ => meta_getter(to)?
        };
        if define.is_status() {
            if instances.len() > 1 {
                return Err(NatureError::ConverterLogicalError("[status meta] must return less 2 instances!".to_string()));
            }
            // status version must equal old + 1
            if instances.len() == 1 {
                let mut ins = instances[0].clone();
                ins.data.status_version += 1;
                ins.data.meta = to.clone();
                rtn.push(ins);
            }
            return Ok(rtn);
        }

        // all biz must same to "to" and set id
        for r in instances {
            let mut instance = r.clone();
            instance.data.meta = to.clone();
            let _ = instance.fix_id();
            rtn.push(instance);
        }

        Ok(rtn)
    }
}