use super::*;

#[test]
fn verify_un_configured() {
    let mut instance = Instance::default();
    instance.data.thing.key = "err".to_string();
    let result = InstanceImpl::verify(&mut instance, Root::Business);
    assert!(result.is_err());
}

#[test]
fn verify_id_generated() {
    let mut instance = Instance::default();
    instance.data.thing.key = "ok".to_string();
    let result = InstanceImpl::verify(&mut instance, Root::Business);
    assert!(result.is_ok());
}

#[test]
fn id_generate() {
    let mut instance = Instance {
        id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        data: InstanceNoID {
            thing: Thing { key: "hello".to_string(), version: 3 },
            execute_time: 0,
            create_time: 0,
            content: String::new(),
            context: HashMap::new(),
            status: HashSet::new(),
        },
    };
    InstanceImpl::id_generate_if_not_set(&mut instance).unwrap();
    println!("{:?}", Uuid::from_bytes(&instance.id));
    assert_eq!(instance.id, [92, 134, 13, 161, 58, 84, 48, 67, 177, 110, 233, 201, 56, 64, 195, 240]);
}
