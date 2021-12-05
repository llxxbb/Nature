use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S, T>(
    date: &T,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize + Display,
{
    serializer.serialize_str(&date.to_string())
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D, T>(
    deserializer: D,
) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr, <T as FromStr>::Err: Display
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod test {
    use crate::util::{is_default, str_2_64};

    #[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Clone, Ord, PartialOrd)]
    struct Test {
        #[serde(skip_serializing_if = "is_default")]
        #[serde(default)]
        #[serde(with = "str_2_64")]
        pub num: u64,
    }

    #[test]
    fn serde() {
        let test = Test {
            num: 100
        };
        let rtn = serde_json::to_string(&test).unwrap();
        assert_eq!(r#"{"num":"100"}"#, rtn);
        let t: Test = serde_json::from_str(r#"{"num":"100"}"#).unwrap();
        assert_eq!(100, t.num);
    }
}