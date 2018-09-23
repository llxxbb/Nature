use flow::convert::local::LocalExecutorTrait;
use nature_common::*;
use nature_db::*;
use std::marker::PhantomData;

pub trait CallOutTrait {
    fn convert(converter_info: &Carrier<ConverterInfo>, para: &CallOutParameter) -> Result<ConverterReturned>;
}

pub struct CallOutImpl<LR> {
    local_rust: PhantomData<LR>,
}

impl<LR> CallOutTrait for CallOutImpl<LR> where LR: LocalExecutorTrait {
    fn convert(converter_info: &Carrier<ConverterInfo>, para: &CallOutParameter) -> Result<ConverterReturned> {
        match converter_info.target.executor.protocol {
            Protocol::LocalRust => {
                Ok(LR::execute(&converter_info.target.executor.url, para))
            }
            _ => {
                // TODO
                unimplemented!()
            }
        }
    }
}
