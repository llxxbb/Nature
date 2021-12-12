use std::ops::Deref;

use crate::retry_lib::cfg::*;

/// Exponential change based on `BASE_DELAY`
/// next delay would be 2^times * `BASE_DELAY`
pub fn get_delay_by_times(times: i16) -> i32 {
    let base = *BASE_DELAY.deref() as i32;
    if times == 0 { return base; }
    (1 << times) * base as i32
}

#[cfg(test)]
mod test {
    use std::env;

    use super::*;

    #[test]
    fn test_get_delay_by_times() {
        env::set_var("BASE_DELAY", "10");
        assert_eq!(get_delay_by_times(0), 10);
        assert_eq!(get_delay_by_times(1), 20);
        assert_eq!(get_delay_by_times(2), 40);
        assert_eq!(get_delay_by_times(3), 80);
        assert_eq!(get_delay_by_times(4), 160);
    }
}