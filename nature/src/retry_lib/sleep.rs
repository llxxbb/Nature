use std::thread::sleep;
use std::time::Duration;

use crate::retry_lib::cfg::*;

/// more recorded less sleep
pub fn sleep_by_records(records: u32, last_sleep: u64) -> u64 {
    let sl = match records {
        0 => {
            let last_sleep = if last_sleep == 0 { 1 } else { last_sleep };
            let last_sleep = last_sleep << 1;
            if last_sleep >= *MAX_SLEEP {
                *MAX_SLEEP
            } else { last_sleep }
        }
        _ => {
            let last_sleep = last_sleep >> 1;
            if last_sleep <= *MIN_SLEEP {
                *MIN_SLEEP
            } else { last_sleep }
        }
    };
    debug!("sleep {} ms", sl);
    sleep(Duration::from_millis(sl));
    sl
}

#[cfg(test)]
mod test {
    use std::env;

    use super::*;

    #[test]
    #[ignore]
    fn sleep_max_test() {
        let _ = env_logger::init();
        env::set_var("MAX_SLEEP", "10");
        env::set_var("MIN_SLEEP", "4");

        assert_eq!(sleep_by_records(0, 0), 2);
        assert_eq!(sleep_by_records(0, 1), 2);
        assert_eq!(sleep_by_records(0, 4), 8);
        assert_eq!(sleep_by_records(0, *MAX_SLEEP), *MAX_SLEEP);
        assert_eq!(sleep_by_records(0, *MAX_SLEEP + 100), *MAX_SLEEP);
        assert_eq!(sleep_by_records(5, 100), 50);
        assert_eq!(sleep_by_records(5, 0), *MIN_SLEEP);
    }
}