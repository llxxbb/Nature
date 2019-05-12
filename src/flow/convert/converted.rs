use super::*;

pub struct Converted {
    pub done_task: RawTask,
    pub converted: Vec<Instance>,
}

impl Converted {
    pub fn gen<FT>(task: &ConverterInfo, carrier: &RawTask, instances: &mut Vec<Instance>, thing_getter: FT) -> Result<Converted>
        where FT: Fn(&Thing) -> Result<RawThingDefine>
    {
        // check `ThingType` for Null
        if task.target.to.get_thing_type() == ThingType::Null {
            let rtn = Converted {
                done_task: carrier.to_owned(),
                converted: Vec::new(),
            };
            return Ok(rtn);
        }
        // check status version to avoid loop
        let _ = instances.iter_mut().map(|one: &mut Instance| {
            one.data.thing = task.target.to.clone();
            let _ = one.fix_id();
            one
        }).collect::<Vec<_>>();
        let instances = Self::verify(&task.target.to, &instances, thing_getter)?;
        let rtn = Converted {
            done_task: carrier.to_owned(),
            converted: instances,
        };
        Ok(rtn)
    }

    fn verify<FT>(to: &Thing, instances: &[Instance], thing_getter: FT) -> Result<Vec<Instance>>
        where FT: Fn(&Thing) -> Result<RawThingDefine>,
    {
        let mut rtn: Vec<Instance> = Vec::new();
        // only one status instance should return
        let define = match to.get_thing_type() {
            ThingType::Dynamic => RawThingDefine::default(),
            // TODO need be replaced
            _ => thing_getter(to)?
        };
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

        // all biz must same to "to" and set id
        for r in instances {
            let mut instance = r.clone();
            instance.data.thing = to.clone();
            let _ = instance.fix_id();
            rtn.push(instance);
        }

        Ok(rtn)
    }
}