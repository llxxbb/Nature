use fg_service::CallOutTrait;
use super::*;

pub trait CallOutTrait {
    fn convert(para: &Carrier<ConverterInfo>) -> Result<ConverterReturned>;
}

pub struct ConvertImpl;

impl CallOutTrait for ConvertImpl {
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
