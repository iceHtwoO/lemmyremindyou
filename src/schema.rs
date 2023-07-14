// @generated automatically by Diesel CLI.

diesel::table! {
    reminder (id) {
        id -> Int4,
        post_id -> Int4,
        parent_id -> Int4,
        #[max_length = 255]
        content -> Varchar,
        #[max_length = 255]
        user_message -> Nullable<Varchar>,
        post_timestamp -> Int8,
        reminder_timestamp -> Int8,
        reminded -> Bool,
    }
}
