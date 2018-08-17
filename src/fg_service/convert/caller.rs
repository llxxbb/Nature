use data::*;
use global::*;
use nature_common::*;

pub trait CallOutTrait {
    fn convert(para: &Carrier<ConverterInfo>) -> Result<ConverterReturned>;
}

pub struct CallOutImpl;

impl CallOutTrait for CallOutImpl {
    fn convert(carrier: &Carrier<ConverterInfo>) -> Result<ConverterReturned> {
        match carrier.target.executor.protocol {
            Protocol::LocalRust => {
                let _para = ConverterInfo::gen_out_parameter(carrier);
                // TODO
                unimplemented!()
            }
            _ => {
                // TODO
                unimplemented!()
            }
        }
    }
}
