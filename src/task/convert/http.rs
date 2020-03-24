use reqwest::blocking::Client;

use nature_common::{ConverterParameter, ConverterReturned};

use crate::task::ExecutorTrait;

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub struct HttpExecutorImpl;

impl ExecutorTrait for HttpExecutorImpl {
    fn execute(&self, address: &str, para: &ConverterParameter) -> ConverterReturned {
        let response = CLIENT.post(address).json(para).send();
        match response {
            Err(e) => ConverterReturned::EnvError(format!("failed get result from : {}, err: {:?}", address, e)),
            Ok(res) => {
                let result = res.json::<ConverterReturned>();
                match result {
                    Err(e) => ConverterReturned::LogicalError(e.to_string()),
                    Ok(ins) => ins
                }
            }
        }
    }
}
