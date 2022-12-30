extern crate nature;

use nature::domain::*;
use nature::common::Result;

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn rtn_none(_para: &ConverterParameter) -> ConverterReturned {
    ConverterReturned::None
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn rtn_logical_error(_para: &ConverterParameter) -> ConverterReturned {
    ConverterReturned::LogicalError { msg: "logical".to_string() }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn rtn_one(_para: &ConverterParameter) -> ConverterReturned {
    let mut instance = Instance::default();
    instance.data.content = "one".to_string();
    ConverterReturned::Instances { ins: vec![instance] }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn rtn_tow(_para: &ConverterParameter) -> ConverterReturned {
    let mut one = Instance::default();
    one.data.content = "one".to_string();
    let mut two = Instance::default();
    two.data.content = "two".to_string();
    ConverterReturned::Instances { ins: vec![one, two] }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn rtn_environment_error(_para: &ConverterParameter) -> ConverterReturned {
    ConverterReturned::EnvError { msg: "aforethought".to_string() }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn convert_before_test(para: &Instance) -> Result<Instance> {
    let mut rtn = para.clone();
    rtn.content = "hello".to_string();
    Ok(rtn)
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn convert_after_test(para: &Vec<Instance>) -> Result<Vec<Instance>> {
    let rtn = para.iter().map(|rtn| {
        let mut rtn = rtn.clone();
        rtn.content = "hello".to_string();
        rtn
    }).collect();
    Ok(rtn)
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn append_star(ins: &Instance) -> Result<Instance> {
    dbg!("----------- append_star ----------");
    let mut ins = ins.clone();
    ins.content = ins.content.to_string() + " *";
    Ok(ins)
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn append_plus(ins: &Instance) -> Result<Instance> {
    dbg!("----------- append_plus ----------");
    let mut ins = ins.clone();
    ins.content = ins.content.to_string() + " +";
    Ok(ins)
}