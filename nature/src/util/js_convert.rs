use crate::domain::common::*;

pub fn try_to_i64(input: Option<String>) -> Result<Option<i64>> {
    match input {
        None => Ok(None),
        Some(val) => Ok(Some(val.parse()?))
    }
}

pub fn long_to_string<T>(input: Result<T>) -> Result<String>
    where T: ToString
{
    match input {
        Err(e) => Err(e),
        Ok(val) => Ok(val.to_string())
    }
}


pub fn to_js_output<T, U>(input: Result<T>) -> Result<U>
    where U: From<T>
{
    match input {
        Err(e) => Err(e),
        Ok(ins) => Ok(ins.into())
    }
}

pub fn to_js_option_output<T, U>(input: Result<Option<T>>) -> Result<Option<U>>
    where U: From<T>
{
    match input {
        Err(e) => Err(e),
        Ok(op) => Ok(match op {
            None => None,
            Some(x) => Some(x.into())
        })
    }
}

pub fn to_js_vec_output<T, U>(input: Result<Vec<T>>) -> Result<Vec<U>>
    where U: From<T>
{
    match input {
        Err(e) => Err(e),
        Ok(ins) => Ok({
            ins.into_iter().map(|one| one.into()).collect()
        })
    }
}
