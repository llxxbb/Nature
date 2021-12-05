use std::fmt::Debug;

use actix_web::HttpResponse;

use crate::domain::Result;

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

pub fn web_result<T>(x: Result<T>) -> HttpResponse
    where T: serde::Serialize + Debug
{
    HttpResponse::Ok().json(x)
}

pub mod str_2_64;

#[cfg(test)]
mod test {
    use crate::domain::*;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_default(&0), true);
        assert_eq!(is_default(&1), false);
        assert_eq!(is_default(&false), true);
        assert_eq!(is_default(&true), false);
        let mut ins = Instance::default();
        assert_eq!(is_default(&ins), true);
        ins.path.para = "hello".to_string();
        assert_eq!(is_default(&ins), false);
    }
}
