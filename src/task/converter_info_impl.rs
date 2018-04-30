use dao::*;
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
                match instance.context.get(&*CONTEXT_STATUS_INSTANCE_ID) {
                    // context have target id
                    Some(status_id) => {
                        let status_id = Uuid::parse_str(status_id)?;
                        InstanceDaoService::get_last_status_by_id(&status_id.as_bytes())?
                    }
                    None => None,
                }
            }
        };
        let rtn = ConverterInfo {
            from: instance.clone(),
            mapping: mapping.clone(),
            last_status: last_target,
        };
        Ok(rtn)
    }
}
