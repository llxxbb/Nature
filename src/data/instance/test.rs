use super::*;

#[test]
fn create_virtual_for_serial_instance(){
    let mut serial = SerialBatchInstance{
        context_for_finish: "all_finished=true".to_string(),
        ignore_error: false,
        instance: Vec::new(),
    };
    let result = Instance::new_batch_for_serial(&mut serial);
    assert!(result.is_ok());
}