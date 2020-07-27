use nature_common::{CONTEXT_LOOP_TASK, Instance, NatureError, Result};
use nature_db::{MetaCache, MetaDao, Mission, MissionRaw};

pub async fn gen_loop_mission<MC, M>(ins: &Instance, mc_g: &MC, m_g: &M) -> Result<Vec<Mission>>
    where MC: MetaCache, M: MetaDao
{
    let option = ins.sys_context.get(CONTEXT_LOOP_TASK);
    if option.is_none() {
        let msg = "can not get loop task from sys_context".to_string();
        error!("{}", msg);
        return Err(NatureError::LogicalError(msg));
    }
    let raw = MissionRaw::from_json(option.unwrap())?;
    let mission = Mission::from_raw(&raw, mc_g, m_g).await?;
    Ok(vec![mission])
}