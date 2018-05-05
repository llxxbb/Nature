use dao::*;
use std::collections::HashSet;
use super::*;
use uuid::Uuid;

impl ConverterInfo {
    /// **Error:**
    /// * Dao
    /// * DefineNotFind
    /// * uuid parse
    pub fn new(instance: &Instance, mapping: &Mapping) -> Result<ConverterInfo> {
        let define = ThingDefine::new(&mapping.to)?;
        let last_target = match define.is_status() {
            false => None,
            true => {
                match instance.context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
                    // context have target id
                    Some(status_id) => {
                        let status_id = Uuid::parse_str(status_id)?;
                        InstanceDaoService::get_last_status_by_id(&status_id.as_bytes())?
                    }
                    None => None,
                }
            }
        };
        if let Some(ref last) = last_target {
            Self::check_last(&last.status, &mapping.demand)?;
        };
        let rtn = ConverterInfo {
            from: instance.clone(),
            mapping: mapping.clone(),
            last_status: last_target,
        };
        Ok(rtn)
    }

    fn check_last(last: &HashSet<String>, demand: &Demand) -> Result<()> {
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
