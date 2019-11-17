use reqwest::Client;

use nature_common::{ConverterParameter, ConverterReturned};

use crate::task::ExecutorTrait;
use futures::executor::block_on;

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub struct HttpExecutorImpl;

impl ExecutorTrait for HttpExecutorImpl {
    fn execute(&self, address: &str, para: &ConverterParameter) -> ConverterReturned {
        let rtn = async {
            let response = CLIENT.post(address).json(para).send();
            match response {
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
            }
        };
        block_on(rtn)
    }
}
