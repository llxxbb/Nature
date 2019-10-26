use reqwest::Client;

use nature_common::{ConverterParameter, ConverterReturned, Instance};

use crate::task::ExecutorTrait;

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub struct HttpExecutorImpl;

impl ExecutorTrait for HttpExecutorImpl {
    fn execute(&self, address: &str, para: &ConverterParameter) -> ConverterReturned {
        let response = CLIENT.post(address).json(para).send();
        match response {
            Err(e) => {
                warn!("failed get result from : {}, err: {:?}", address, e);
                ConverterReturned::EnvError
            }
            Ok(mut res) => {
                let result = res.json::<Vec<Instance>>();
                match result {
                    Err(e) => ConverterReturned::LogicalError(e.to_string()),
                    Ok(ins) => ConverterReturned::Instances(ins)
                }
            }
        }
    }
}
