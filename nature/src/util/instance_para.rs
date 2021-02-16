use std::panic::catch_unwind;

use crate::domain::*;
use crate::util::*;

pub fn append_para(para: &str, part: &str) -> String {
    if para.is_empty() {
        return part.to_string();
    }
    if part.is_empty() {
        return para.to_string();
    }
    para.to_string() + &SEPARATOR_INS_PARA + part
}

/// The Ok returned:
/// - .0 : selected
/// - .1 : remained key
pub fn get_para_and_key_from_para(para: &str, part: &Vec<u8>) -> Result<(String, String)> {
    // handle empty
    if part.len() == 0 {
        return Ok(("".to_string(), "".to_string()));
    }
    let sep: &str = &*SEPARATOR_INS_PARA;
    let keys: Vec<&str> = para.split(&sep).collect();
    make_key_and_para(&keys, part, &sep)
}

/// extract String from para by given part
pub fn get_para_part(para: &str, part: &Vec<u8>) -> Result<Vec<String>> {
    // handle empty
    let sep: &str = &*SEPARATOR_INS_PARA;
    let keys: Vec<&str> = para.split(&sep).collect();
    let mut rtn: Vec<String> = Vec::with_capacity(part.len());
    for index in part {
        match catch_unwind(|| { keys[*index as usize] }) {
            Err(e) => {
                let msg = format!("index out of range: {:?}", e);
                warn!("{}", &msg);
                return Err(NatureError::VerifyError(msg));
            }
            Ok(p) => rtn.push(p.to_string())
        };
    }
    Ok(rtn)
}

/// key for instance'content, para for instance's para
/// The Ok returned:
/// - .0 : selected
/// - .1 : remained key
pub fn make_key_and_para(keys: &Vec<&str>, k_index: &Vec<u8>, sep: &str) -> Result<(String, String)> {

    // make instance's para
    let mut p: Vec<&str> = vec![];
    for index in k_index {
        let index = *index as usize;
        if index >= keys.len() {
            return Err(NatureError::VerifyError("outbound index".to_string()));
        }
        p.push(keys[index]);
        p.push(sep);
    }
    let p = p[..p.len() - 1].concat();

    // make key
    let mut k: Vec<&str> = vec![];
    for i in 0..keys.len() {
        if k_index.contains(&(i as u8)) {
            continue;
        }
        k.push(keys[i]);
        k.push(sep);
    }
    let k = match k.len() {
        0 => "".to_string(),
        _ => k[..k.len() - 1].concat()
    };
    Ok((p, k))
}

#[cfg(test)]
mod append_para_test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("", append_para("", ""));
        assert_eq!("a", append_para("a", ""));
        assert_eq!("a", append_para("", "a"));
        assert_eq!("a/b", append_para("a", "b"));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_para_part_test() {
        let result = get_para_part("", &vec![10]);
        assert_eq!(true, result.is_err());

        let result = get_para_part("a/b/c", &vec![1, 0]).unwrap();
        assert_eq!(2, result.len());
        assert_eq!("b", result[0]);
        assert_eq!("a", result[1]);
    }

    #[test]
    fn key_para_make() {
        let keys = vec!["a", "b", "c", "d", "e"];
        let idx = vec![3, 1];
        let result = make_key_and_para(&keys, &idx, "-").unwrap();
        assert_eq!(result.0, "d-b");
        assert_eq!(result.1, "a-c-e");
    }

    #[test]
    fn empty_para() {
        let result = get_para_and_key_from_para("", &vec![]).unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, "");
        let result = get_para_and_key_from_para("a,b,c", &vec![]).unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, "");
    }

    #[test]
    fn normal_test() {
        let result = get_para_and_key_from_para("a/b/c", &vec![0]).unwrap();
        assert_eq!(result.0, "a");
        assert_eq!(result.1, "b/c");
        let result = get_para_and_key_from_para("a/b/c", &vec![1]).unwrap();
        assert_eq!(result.0, "b");
        assert_eq!(result.1, "a/c");
        let result = get_para_and_key_from_para("a/b/c", &vec![2]).unwrap();
        assert_eq!(result.0, "c");
        assert_eq!(result.1, "a/b");
        let result = get_para_and_key_from_para("a/b/c", &vec![0, 1]).unwrap();
        assert_eq!(result.0, "a/b");
        assert_eq!(result.1, "c");
        let result = get_para_and_key_from_para("a/b/c", &vec![1, 2]).unwrap();
        assert_eq!(result.0, "b/c");
        assert_eq!(result.1, "a");
        let result = get_para_and_key_from_para("a/b/c", &vec![0, 2]).unwrap();
        assert_eq!(result.0, "a/c");
        assert_eq!(result.1, "b");

        let result = get_para_and_key_from_para("a/b/c", &vec![1, 0]).unwrap();
        assert_eq!(result.0, "b/a");
        assert_eq!(result.1, "c");
        let result = get_para_and_key_from_para("a/b/c", &vec![2, 1]).unwrap();
        assert_eq!(result.0, "c/b");
        assert_eq!(result.1, "a");
        let result = get_para_and_key_from_para("a/b/c", &vec![2, 0]).unwrap();
        assert_eq!(result.0, "c/a");
        assert_eq!(result.1, "b");
    }
}