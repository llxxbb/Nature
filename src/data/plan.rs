use nature_common::*;

/// **unique key**
/// * from_id
/// * from_thing
#[derive(Debug)]
pub struct PlanInfo {
    pub from_thing: Thing,
    pub from_sn: u128,
    pub from_sta_ver: i32,
    pub to: Thing,
    pub plan: Vec<Instance>,
}
