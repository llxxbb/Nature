use super::*;


pub struct CarrierDaoService;

impl CarrierDao for CarrierDaoService {
    fn insert<T>(_carrier: &Carrier<T>) -> Result<UuidBytes> {
        unimplemented!()
    }
}