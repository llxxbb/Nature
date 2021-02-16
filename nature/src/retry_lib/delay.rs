use std::ops::Deref;

use crate::retry_lib::cfg::*;

/// Exponential change based on `FIRST_RETRY_INTERVAL`
pub fn get_delay_by_times(times: i16) -> i32 {
    if times == 0 { return *FIRST_RETRY_INTERVAL.deref() as i32; }
    (1 << times) * 5 as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_delay_by_times() {
        assert_eq!(get_delay_by_times(0), 5);
        assert_eq!(get_delay_by_times(1), 10);
        assert_eq!(get_delay_by_times(2), 20);
        assert_eq!(get_delay_by_times(3), 40);
        assert_eq!(get_delay_by_times(4), 80);
    }
}