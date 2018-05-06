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
