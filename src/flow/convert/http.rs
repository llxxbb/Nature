use nature_common::{CallOutParameter, ConverterReturned};

use crate::flow::convert::caller::ExecutorTrait;

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub struct HttpExecutorImpl;

impl ExecutorTrait for HttpExecutorImpl {
    fn execute(&self, address: &str, para: &CallOutParameter) -> ConverterReturned {
        unimplemented!()
    }
}
