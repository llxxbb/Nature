use super::schema::one_step_flow;

#[derive(Debug)]
#[derive(Insertable,Queryable)]
#[table_name = "one_step_flow"]
pub struct OneStepFlowRow {
    pub from_thing: String,
    pub from_version: i32,
    pub to_thing: String,
    pub to_version: i32,
    pub exe_protocol: String,
    pub exe_url: String,
    pub selector: Option<String>,
    pub group: Option<String>,
    pub weight: Option<i32>,
}