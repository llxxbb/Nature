table! {
    thing_defines (key) {
        key -> Text,
        description -> Nullable<Text>,
        version -> Integer,
        have_states -> Integer,
        states -> Nullable<Text>,
        fields -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
    }
}
