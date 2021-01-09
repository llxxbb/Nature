use futures::future::BoxFuture;
use futures::FutureExt;

use crate::domain::*;
use crate::middleware::filter::builtin_filter::BuiltIn;
use crate::task::local_common::local_execute;

pub fn convert_before(para: &mut Instance, filter: Vec<Executor>) -> BoxFuture<Result<()>> {
    async move {
        for f in filter {
            match f.protocol {
                Protocol::Http => {
                    http_filter::http_filter(&f.url, para).await?;
                }
                Protocol::LocalRust => {
                    match local_execute(&f.url, para).await {
                        Ok(new) => {
                            match new {
                                Ok(new) => *para = new,
                                Err(err) => return Err(err)
                            }
                        }
                        Err(err) => {
                            warn!("local filter occur error: {}", err);
                            return Err(err);
                        }
                    }
                }
                Protocol::BuiltIn => {
                    let bf = BuiltIn::get(&f.url)?;
                    bf.filter(para, &f.settings).await?;
                }
                _ => return Err(NatureError::VerifyError("filter does not support this protocol".to_string()))
            }
        }
        Ok(())
    }.boxed()
}

pub async fn convert_after(para: &mut Vec<Instance>, filter: &Vec<Executor>) -> Result<()>
{
    for f in filter {
        match f.protocol {
            Protocol::Http => {
                http_filter::http_filter(&f.url, para).await?;
            }
            Protocol::LocalRust => {
                match local_execute(&f.url, para).await {
                    Ok(new) => match new {
                        Ok(new) => *para = new,
                        Err(err) => return Err(err)
                    },
                    Err(err) => {
                        warn!("local filter occur error: {}", err);
                        return Err(err);
                    }
                }
            }
            _ => return Err(NatureError::VerifyError("filter does not support this protocol".to_string()))
        }
    }
    Ok(())
}

mod http_filter;
pub mod builtin_filter;

