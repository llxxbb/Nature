use define::Instance;
use define::InstanceNoID;
use define::Thing;
use service::NatureService;

#[test]
fn id_generate() {
    let svr = NatureService;
    let instance = Instance {
        id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        data: InstanceNoID {
            thing: Thing { key: "hello".to_string(), version: 3 },
            execute_time: 0,
            create_time: 0,
            content: String::new(),
            context: String::new(),
        },
    };
    let result = svr.id_generate_if_not_set(instance).unwrap();
    assert_eq!(result.id, [92, 134, 13, 161, 58, 84, 48, 67, 177, 110, 233, 201, 56, 64, 195, 240]);
}

#[test]
fn id_ignore() {
    let svr = NatureService;
    let instance = Instance {
        id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, ],
        data: InstanceNoID {
            thing: Thing { key: "hello".to_string(), version: 3 },
            execute_time: 0,
            create_time: 0,
            content: String::new(),
            context: String::new(),
        },
    };
    let result = svr.id_generate_if_not_set(instance).unwrap();
    assert_eq!(result.id, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0]);
}

