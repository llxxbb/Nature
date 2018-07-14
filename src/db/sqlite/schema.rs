table! {
    delivery (id) {
        id -> Binary,
        thing -> Text,
        data_type -> SmallInt,
        data -> Text,
        create_time -> Timestamp,
        execute_time -> Timestamp,
        retried_times -> SmallInt,
    }
}

table! {
    delivery_error (id) {
        id -> Binary,
        thing -> Text,
        data_type -> SmallInt,
        data -> Text,
        create_time -> Timestamp,
        msg -> Text,
    }
}

table! {
    instances (id, thing, version, status_version) {
        id -> Binary,
        thing -> Text,
        version -> Integer,
        content -> Text,
        context -> Nullable<Text>,
        status -> Nullable<Text>,
        status_version -> Integer,
        from_thing -> Nullable<Text>,
        from_version -> Nullable<Integer>,
        from_status_version -> Nullable<Integer>,
        event_time -> Timestamp,
        execute_time -> Timestamp,
        create_time -> Timestamp,
    }
}

table! {
    one_step_flow (from_thing, from_version, to_thing, to_version) {
        from_thing -> Text,
        from_version -> Integer,
        to_thing -> Text,
        to_version -> Integer,
        exe_protocol -> Text,
        exe_url -> Text,
        selector -> Nullable<Text>,
        group -> Nullable<Text>,
        weight -> Nullable<Integer>,
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
    delivery,
    delivery_error,
    instances,
    one_step_flow,
    thing_defines,
);
