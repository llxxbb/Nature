table! {
    instances (id) {
        id -> Integer,
        thing -> Text,
        version -> Integer,
        content -> Nullable<Text>,
        context -> Nullable<Text>,
        status -> Nullable<Text>,
        status_version -> Integer,
        from_thing -> Nullable<Text>,
        from_version -> Nullable<Integer>,
        from_status_version -> Nullable<Integer>,
        execute_time -> Timestamp,
        create_time -> Timestamp,
    }
}

table! {
    thing_defines (key, version) {
        key -> Text,
        description -> Nullable<Text>,
        version -> Integer,
        states -> Nullable<Text>,
        fields -> Nullable<Text>,
        create_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    instances,
    thing_defines,
);
