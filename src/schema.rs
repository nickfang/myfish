// @generated automatically by Diesel CLI.

diesel::table! {
    assets (id) {
        id -> Uuid,
        name -> Varchar,
        value -> Int4,
        description -> Nullable<Text>,
        longterm -> Bool,
        date_acquired -> Nullable<Date>,
        date_divested -> Nullable<Date>,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    liabilities (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        amount -> Nullable<Int4>,
        payee -> Nullable<Varchar>,
        payee_info -> Nullable<Text>,
        longterm -> Nullable<Bool>,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        date -> Date,
        name -> Varchar,
        amount -> Nullable<Int4>,
        transaction_type -> Varchar,
        category -> Varchar,
        description -> Text,
        note -> Nullable<Varchar>,
        user_id -> Uuid,
        asset_id -> Nullable<Uuid>,
        liability_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(assets -> users (user_id));
diesel::joinable!(liabilities -> users (user_id));
diesel::joinable!(transactions -> assets (asset_id));
diesel::joinable!(transactions -> liabilities (liability_id));
diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    assets,
    liabilities,
    transactions,
    users,
);
