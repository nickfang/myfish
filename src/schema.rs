// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Int4,
        date -> Date,
        name -> Text,
        amount -> Numeric,
        transaction_type -> Text,
        category -> Text,
        #[max_length = 255]
        description -> Varchar,
        note -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
