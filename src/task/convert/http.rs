use reqwest::Client;

use nature_common::{ConverterParameter, ConverterReturned};

use crate::task::ExecutorTrait;

pub struct HttpExecutorImpl;

impl ExecutorTrait for HttpExecutorImpl {
    fn execute(&self, address: &str, para: &ConverterParameter) -> ConverterReturned {
        let client = Client::new();
        let response = client.post(address).json(para).send();
        let rtn = match response {
            Err(e) => {
                warn!("failed get result from : {}, err: {:?}", address, e);
                ConverterReturned::EnvError
            }
            Ok(mut res) => {
                let result = res.json::<ConverterReturned>();
                match result {
                    Err(e) => ConverterReturned::LogicalError(e.to_string()),
                    Ok(ins) => ins
                }
            }
        };
        rtn
    }
}
