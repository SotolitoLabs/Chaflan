// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Uuid,
        name -> Text,
        venue -> Text,
        address -> Nullable<Text>,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        comments -> Nullable<Text>,
        #[max_length = 255]
        contactname -> Nullable<Varchar>,
        starts_at -> Timestamp,
        ends_at -> Timestamp,
    }
}
