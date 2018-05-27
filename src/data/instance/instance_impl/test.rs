use super::*;
use uuid::*;

#[test]
fn verify_un_configured() {
    println!("----------------- verify_un_configured --------------------");
    let mut instance = Instance::default();
    instance.data.thing.key = "err".to_string();
    let result = InstanceImpl::verify(&mut instance, Root::Business);
    assert!(result.is_err());
}

#[test]
fn verify_id_generated() {
    println!("----------------- verify_id_generated --------------------");
    let mut instance = Instance::default();
    instance.data.thing.key = "/id/test".to_string();
    let _lock_define_cache = lock_and_set_mock_value(&THING_DEFINE_LOCK, &THING_DEFINE_CACHE_VALUE, Ok(ThingDefine::default()));
    let result = InstanceImpl::verify(&mut instance, Root::Business);
    assert!(result.is_ok());
}

#[test]
fn id_generate() {
    println!("----------------- id_generate --------------------");
    let mut instance = Instance {
        id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        data: InstanceNoID {
            thing: Thing { key: "hello".to_string(), version: 3 },
            execute_time: 0,
            create_time: 0,
            content: String::new(),
            context: HashMap::new(),
            status: HashSet::new(),
            status_version: 0,
            from: None,
        },
    };
    InstanceImpl::id_generate_if_not_set(&mut instance).unwrap();
    println!("{:?}", Uuid::from_bytes(&instance.id));
    assert_eq!(instance.id, [124, 65, 30, 60, 106, 24, 52, 15, 171, 142, 92, 88, 85, 237, 50, 220]);
}
