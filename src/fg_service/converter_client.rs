use fg_service::CallOutTrait;
use super::*;


pub struct ConvertImpl;

impl CallOutTrait for ConvertImpl {
    fn convert(carrier: &Carrier<ConverterInfo>) -> Result<ConverterReturned> {
        let _para = ConverterInfo::to_out_parameter(carrier);

        unimplemented!()
    }
}
