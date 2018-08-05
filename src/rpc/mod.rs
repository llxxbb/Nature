use data::*;
use global::Result;
///! rpc server, collect data from different rpc client then call the server

pub use self::rocket::*;


pub mod rocket;

pub trait CallOutTrait {
    fn convert(para: &Carrier<ConverterInfo>) -> Result<ConverterReturned>;
}


