use data::*;
use flow::LocalExecutorTrait;
use global::*;
use nature_common::*;
use std::marker::PhantomData;

pub trait CallOutTrait {
    fn convert(para: &Carrier<ConverterInfo>) -> Result<ConverterReturned>;
}

pub struct CallOutImpl<LR> {
    local_rust: PhantomData<LR>
}

impl<LR> CallOutTrait for CallOutImpl<LR> where LR: LocalExecutorTrait {
    fn convert(carrier: &Carrier<ConverterInfo>) -> Result<ConverterReturned> {
        match carrier.target.executor.protocol {
            Protocol::LocalRust => {
                let para = ConverterInfo::gen_out_parameter(carrier);
                Ok(LR::execute(&carrier.target.executor.url, &para))
            }
            _ => {
                // TODO
                unimplemented!()
            }
        }
    }
}
