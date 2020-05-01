use chrono::{Datelike, Local, NaiveDateTime, Timelike, TimeZone};

use nature_common::{ConverterParameter, ConverterReturned, Instance, is_default, NatureError, Result};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Setting {
    /// s(econd), m(inute), h(our), d(ay)
    #[serde(skip_serializing_if = "is_s")]
    #[serde(default = "string_s")]
    unit: String,
    /// When unit is s,m,h,d the value great than 1, it mean interval
    /// When unit is w,M,y the value mean offset. It could be negative, means offset from the end. 0 is the first day of the unit.
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    value: i16,
}

static SECOND: i64 = 1000;
static MINUTE: i64 = 1000 * 60;
static HOUR: i64 = 1000 * 60 * 60;
static DAY: i64 = 1000 * 60 * 60 * 24;

impl Setting {
    fn get_time(&self, ins_time: i64) -> Result<i64> {
        let time = Local.timestamp_millis(ins_time).naive_local();
        let unit = self.unit.as_ref();
        let rtn = match unit {
            "s" => ins_time - time.timestamp_subsec_millis() as i64,
            "m" => ins_time - time.second() as i64 * SECOND - time.timestamp_subsec_millis() as i64,
            "h" => ins_time - time.minute() as i64 * MINUTE - time.second() as i64 * SECOND - time.timestamp_subsec_millis() as i64,
            "d" => ins_time - time.hour() as i64 * HOUR - time.minute() as i64 * MINUTE - time.second() as i64 * SECOND - time.timestamp_subsec_millis() as i64,
            "w" => return self.get_week(ins_time, &time),
            _ => {
                let err = format!("timer setting error: unknown unit '{}'", self.unit);
                return Err(NatureError::LogicalError(err));
            }
        };
        let rtn = if self.value > 1 {
            match unit {
                "s" => {
                    let redundant = time.second() % self.value as u32;
                    rtn - redundant as i64 * SECOND
                }
                "m" => {
                    let redundant = time.minute() % self.value as u32;
                    rtn - redundant as i64 * MINUTE
                }
                "h" => {
                    let redundant = time.hour() % self.value as u32;
                    rtn - redundant as i64 * HOUR
                }
                "d" => {
                    let redundant = time.num_days_from_ce() % self.value as i32;
                    rtn - redundant as i64 * DAY
                }
                _ => {
                    let err = format!("timer setting error: unknown unit '{}'", self.unit);
                    return Err(NatureError::LogicalError(err));
                }
            }
        } else {
            rtn
        };
        Ok(rtn)
    }

    fn get_week(&self, real: i64, nd: &NaiveDateTime) -> Result<i64> {
        if self.value > 6 || self.value < -7 {
            return Err(NatureError::LogicalError("value must in [-7,6]".to_string()));
        }
        let offset = nd.weekday().num_days_from_monday() as i16;
        let mut value = self.value;
        if value < 0 {
            value += 7
        }
        let diff_day = if value < offset {
            offset - value
        } else {
            7 - value + offset
        };
        let rtn = real - diff_day as i64 * DAY - nd.hour() as i64 * HOUR - nd.minute() as i64 * MINUTE - nd.second() as i64 * SECOND - nd.timestamp_subsec_millis() as i64;
        Ok(rtn)
    }

    // fn get_month(&self, real: i64, nd: &NaiveDateTime) -> Result<i64> {
    //     let offset = nd.day0();
    //     let mut value = self.value;
    //     if value < 0 {
    //         value += nd.m
    //     }
    //     let diff_day = if value < offset {
    //         offset - value
    //     } else {
    //         7 - value + offset
    //     };
    //     let rtn = real - diff_day as i64 * DAY - nd.hour() as i64 * HOUR - nd.minute() as i64 * MINUTE - nd.second() as i64 * SECOND - nd.timestamp_subsec_millis() as i64;
    //     Ok(rtn)
    // }
}


/// generate a timer para
pub fn timer(input: &ConverterParameter) -> ConverterReturned {
    // get setting
    let cfg = match serde_json::from_str::<Setting>(&input.cfg) {
        Ok(cfg) => cfg,
        Err(err) => {
            warn!("error setting: {}", &input.cfg);
            return ConverterReturned::LogicalError(err.to_string());
        }
    };
    let result = match cfg.get_time(input.from.create_time) {
        Ok(rtn) => rtn,
        Err(err) => return ConverterReturned::LogicalError(err.to_string())
    };
    let mut instance = Instance::default();
    instance.para = result.to_string();
    ConverterReturned::Instances(vec![instance])
}

fn is_s(cmp: &str) -> bool {
    cmp.eq("s")
}

fn string_s() -> String {
    "s".to_string()
}

#[cfg(test)]
mod timer_setting_test {
    use super::*;

    #[test]
    fn test() {
        let mut setting = Setting {
            unit: "s".to_string(),
            value: 0,
        };
        let rtn = serde_json::to_string(&setting).unwrap();
        assert_eq!(rtn, "{}");
        let rtn: Setting = serde_json::from_str("{}").unwrap();
        assert_eq!(rtn, setting);
        setting.unit = "a".to_string();
        setting.value = 100;
        let rtn = serde_json::to_string(&setting).unwrap();
        let json = r#"{"unit":"a","value":100}"#;
        assert_eq!(rtn, json);
        let rtn: Setting = serde_json::from_str(json).unwrap();
        assert_eq!(rtn, setting);
    }

    #[test]
    fn second_test() {
        let time = Local.ymd(2020, 5, 1).and_hms_milli(18, 36, 23, 123).timestamp_millis();
        let mut setting = Setting {
            unit: "s".to_string(),
            value: 0,
        };
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 5, 1).and_hms(18, 36, 23).timestamp_millis();
        assert_eq!(rtn, cmp);
        // test interval
        setting.value = 15;
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 5, 1).and_hms(18, 36, 15).timestamp_millis();
        assert_eq!(rtn, cmp);
    }

    #[test]
    fn minute_test() {
        let time = Local.ymd(2020, 5, 1).and_hms_milli(18, 36, 23, 123).timestamp_millis();
        let mut setting = Setting {
            unit: "m".to_string(),
            value: 0,
        };
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 5, 1).and_hms(18, 36, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
        // test interval
        setting.value = 10;
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 5, 1).and_hms(18, 30, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
    }

    #[test]
    fn hour_test() {
        let time = Local.ymd(2020, 5, 1).and_hms_milli(18, 36, 23, 123).timestamp_millis();
        let mut setting = Setting {
            unit: "h".to_string(),
            value: 0,
        };
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 5, 1).and_hms(18, 0, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
        // test interval
        setting.value = 4;
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 5, 1).and_hms(16, 0, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
    }

    #[test]
    fn day_test() {
        let time = Local.ymd(2020, 5, 1).and_hms_milli(18, 36, 23, 123).timestamp_millis();
        let mut setting = Setting {
            unit: "d".to_string(),
            value: 0,
        };
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 5, 1).and_hms(0, 0, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
        // test interval
        setting.value = 3;
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 4, 29).and_hms(0, 0, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
    }

    #[test]
    fn week_test() {
        // the `value` is positive and before the real time
        let time = Local.ymd(2020, 5, 1).and_hms_milli(18, 36, 23, 123).timestamp_millis();
        let mut setting = Setting {
            unit: "w".to_string(),
            value: 0,
        };
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 4, 27).and_hms(0, 0, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
        // the `value` is positive and after the real time
        setting.value = 6;
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 4, 26).and_hms(0, 0, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
        // the `value` is negative and before the real time
        setting.value = -7;
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 4, 27).and_hms(0, 0, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
        // the `value` is negative and after the real time
        setting.value = -1;
        let rtn = setting.get_time(time).unwrap();
        let cmp = Local.ymd(2020, 4, 26).and_hms(0, 0, 0).timestamp_millis();
        assert_eq!(rtn, cmp);
        // error input
        setting.value = 7;
        let rtn = setting.get_time(time);
        assert!(rtn.is_err());
        setting.value = -8;
        let rtn = setting.get_time(time);
        assert!(rtn.is_err());
    }
}

