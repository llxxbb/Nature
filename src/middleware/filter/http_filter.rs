use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::domain::*;

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub async fn http_filter<T: Serialize + DeserializeOwned>(address: &str, para: &mut T) -> Result<()> {
    let rtn = CLIENT.post(address).json(para).send().await?.json::<Result<T>>().await?;
    match rtn {
        Ok(o) => {
            *para = o;
            Ok(())
        }
        Err(e) => {
            warn!("filter occur error: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;

    use super::*;

    /// when run this test please run project `nature_demo_executor_restful` first
    // #[test]
    #[allow(dead_code)]
    fn http_filter_test_ok() {
        let mut ins = Instance::default();
        ins.para = "/subject2".to_string();
        ins.content = "100".to_string();
        let mut input: Vec<Instance> = vec![];
        input.push(ins);
        let mut runtime = Runtime::new().unwrap();
        let _rtn = runtime.block_on(http_filter("http://127.0.0.1:8082/add_score", &mut input));
        assert_eq!(input[0].content, "104");
    }

    #[test]
    fn http_filter_test_remote_unusable() {
        let mut ins = Instance::default();
        ins.para = "/subject2".to_string();
        ins.content = "100".to_string();
        let mut input: Vec<Instance> = vec![];
        input.push(ins);
        let mut runtime = Runtime::new().unwrap();
        let rtn = runtime.block_on(http_filter("http://error.com", &mut input));
        dbg!(&rtn);
        assert!(rtn.is_err());
    }
}

