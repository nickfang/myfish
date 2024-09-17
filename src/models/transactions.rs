use serde::{ Deserialize, Serialize };
use diesel::{ Queryable, Insertable, AsChangeset };
use uuid::Uuid;

use chrono::{ NaiveDateTime, NaiveDate };
use crate::schema::transactions;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub date: NaiveDate,
    pub name: String,
    pub amount: i32,
    pub transaction_type: String,
    pub category: String,
    pub description: String,
    pub note: Option<String>,
    pub user_id: Uuid,
    pub asset_id: Option<Uuid>,
    pub liabilities_id: Option<Uuid>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub date: NaiveDate,
    pub name: String,
    pub amount: i32,
    pub transaction_type: String,
    pub category: String,
    pub description: String,
    pub note: Option<String>,
    pub user_id: Option<Uuid>,
    pub asset_id: Option<Uuid>,
    pub liability_id: Option<Uuid>,
}
