/// This is only used for serialize
pub fn is_one(num: &i32) -> bool {
    *num == 1
}

pub fn one() -> i32 { 1 }

pub fn is_one_u32(num: &u32) -> bool {
    *num == 1
}

pub fn one_u32() -> u32 { 1 }

pub fn is_default<T: Default + Eq>(val: &T) -> bool {
    val.eq(&T::default())
}

#[cfg(test)]
mod test {
    use crate::common::Instance;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_default(&0), true);
        assert_eq!(is_default(&1), false);
        assert_eq!(is_default(&false), true);
        assert_eq!(is_default(&true), false);
        let mut ins = Instance::default();
        assert_eq!(is_default(&ins), true);
        ins.para = "hello".to_string();
        assert_eq!(is_default(&ins), false);
    }
}
