use reqwest::Client;

use nature_common::{ConverterParameter, ConverterReturned, NatureError, Result};

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub async fn http_execute_async(address: &str, para: &ConverterParameter) -> ConverterReturned {
    let rtn = reqwest_call(address, para).await;
    match rtn {
        Ok(e) => e,
        Err(e) => ConverterReturned::EnvError(e.to_string())
    }
}

async fn reqwest_call(address: &str, para: &ConverterParameter) -> Result<ConverterReturned> {
    match CLIENT.post(address).json(para).send().await?.json::<ConverterReturned>().await {
        Ok(o) => Ok(o),
        Err(e) => Err(NatureError::from(e))
    }
}

