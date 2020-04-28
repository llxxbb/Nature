use reqwest::Client;

use nature_common::{Instance, NatureError, Result};

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub async fn http_filter_after(address: &str, para: &mut Vec<Instance>) -> Result<()> {
    let rtn = CLIENT.post(address).json(para).send().await?.json::<Result<Vec<Instance>>>().await?;
    match rtn {
        Ok(o) => {
            *para = o;
            Ok(())
        }
        Err(e) => Err(NatureError::from(e))
    }
}

