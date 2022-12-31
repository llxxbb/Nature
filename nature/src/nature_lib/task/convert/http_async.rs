use reqwest::Client;
use crate::common::*;

use crate::domain::*;

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub async fn http_execute_async(address: &str, para: &ConverterParameter) -> ConverterReturned {
    let rtn = reqwest_call(address, para).await;
    match rtn {
        Ok(e) => e,
        Err(e) => ConverterReturned::EnvError { msg: e.to_string() }
    }
}

async fn reqwest_call(address: &str, para: &ConverterParameter) -> Result<ConverterReturned> {
    match CLIENT.post(address).json(para).send().await?.json::<ConverterReturned>().await {
        Ok(o) => Ok(o),
        Err(e) => Err(NatureError::from(e))
    }
}

