use dao::*;
use super::*;
use uuid::Uuid;

impl ParaForCallOut {
    /// **Error:**
    /// * Dao
    /// * DefineNotFind
    /// * uuid parse
    ///
    pub fn new(converter_info: &ConverterInfo) -> Result<ParaForCallOut> {
        let define = ThingDefine::new(converter_info.mapping.to.clone())?;
        // target is a no status thing
        if !define.is_status() {
            return Ok(ParaForCallOut {
                from: converter_info.from.clone(),
                last_target: None,
                for_callback: Vec::new(),
            });
        }
        // target is a status thing
        match converter_info.from.data.context.clone().get(&*CONTEXT_STATUS_INSTANCE_ID) {
            // context have target id
            Some(status_id) => {
                let status_id = Uuid::parse_str(status_id)?;
                let last_target = InstanceDaoService::get_last_status_by_id(&status_id.as_bytes())?;
                Ok(ParaForCallOut {
                    from: converter_info.from.clone(),
                    last_target,
                    for_callback: Vec::new(),
                })
            }
            // no need to prepare last_target
            None => Ok(ParaForCallOut {
                from: converter_info.from.clone(),
                last_target: None,
                for_callback: Vec::new(),
            })
        }
    }
}
