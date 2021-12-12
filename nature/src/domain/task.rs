use crate::util::{is_default, str_2_64};

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Clone, Ord, PartialOrd)]
pub struct TaskCondition {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub task_for: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    #[serde(with = "str_2_64")]
    pub id_from: u64,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub limit: u32,
}

#[cfg(test)]
mod test{
    use crate::domain::task::TaskCondition;

    #[test]
    fn task_condition_test(){
        let one = TaskCondition{
            task_for: "".to_string(),
            id_from: 0,
            limit: 0
        };
        let rtn = serde_json::to_string(&one);
        assert_eq!("{}", rtn.unwrap());
        let two = serde_json::from_str("{}");
        assert_eq!(one, two.unwrap());
    }
}