use super::*;

#[test]
fn id_generate() {
    let mut instance = Instance {
        id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        data: InstanceNoID {
            thing: Thing { key: "hello".to_string(), version: 3 },
            execute_time: 0,
            create_time: 0,
            content: String::new(),
            context: String::new(),
        },
    };
    instance.id_generate_if_not_set().unwrap();
    println!("{:?}", Uuid::from_bytes(&instance.id));
    assert_eq!(instance.id, [92, 134, 13, 161, 58, 84, 48, 67, 177, 110, 233, 201, 56, 64, 195, 240]);
}
