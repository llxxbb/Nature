use http_filter::http_filter;
use nature_common::{Executor, Instance, NatureError, Protocol, Result};

use crate::task::local_common::local_execute;

pub async fn filter_instance(para: &mut Vec<Instance>, filter: &Vec<Executor>) -> Result<()> {
    for f in filter {
        match f.protocol {
            Protocol::Http => {
                let _ = http_filter(&f.url, para).await;
            }
            Protocol::LocalRust => {
                match local_execute(&f.url, para).await {
                    Ok(new) => match new {
                        Ok(new) => *para = new,
                        Err(err) => return Err(err)
                    },
                    Err(err) => return Err(err)
                }
            }
            _ => return Err(NatureError::VerifyError("filter does not support this protocol".to_string()))
        }
    }
    Ok(())
}

mod http_filter;
