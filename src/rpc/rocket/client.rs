use super::*;


pub trait CallOutTrait {
    fn convert(para: CallOutParameter) -> Result<ConverterReturned>;
}

pub struct ConvertImpl;

impl CallOutTrait for ConvertImpl {
    fn convert(_para: CallOutParameter) -> Result<ConverterReturned> {
        unimplemented!()
    }
}
