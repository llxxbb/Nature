use nature_common::{ConverterParameter, ConverterReturned};

use crate::task::ExecutorTrait;

pub struct Counter;

impl ExecutorTrait for Counter {
    fn execute(&self, _executor: &str, _para: &ConverterParameter) -> ConverterReturned {
        // TODO
        unimplemented!();
    }
}

