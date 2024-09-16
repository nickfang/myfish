use serde::{ Deserialize, Serialize };
use diesel::{ Queryable, Insertable, AsChangeset };
use chrono::NaiveDateTime;
use crate::schema::transactions;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub date: chrono::NaiveDate,
    pub name: String,
    pub amount: i32,
    pub transaction_type: String,
    pub category: String,
    pub description: String,
    pub note: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub date: chrono::NaiveDate,
    pub name: String,
    pub amount: i32,
    pub transaction_type: String,
    pub category: String,
    pub description: String,
    pub note: Option<String>,
}
