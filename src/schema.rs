// @generated automatically by Diesel CLI.

diesel::table! {
    assets (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        value -> Nullable<Int4>,
        description -> Nullable<Text>,
        longterm -> Nullable<Bool>,
        date_acquired -> Nullable<Date>,
        date_divested -> Nullable<Date>,
        user_id -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    liabilities (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        amount -> Nullable<Int4>,
        payee -> Nullable<Varchar>,
        payee_info -> Nullable<Text>,
        longterm -> Nullable<Bool>,
        user_id -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        date -> Nullable<Timestamp>,
        name -> Nullable<Varchar>,
        amount -> Nullable<Int4>,
        transaction_type -> Nullable<Varchar>,
        category -> Nullable<Varchar>,
        description -> Nullable<Text>,
        note -> Nullable<Varchar>,
        user_id -> Nullable<Uuid>,
        asset_id -> Nullable<Uuid>,
        liability_id -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
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
