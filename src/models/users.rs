use serde::{ Deserialize, Serialize };
use diesel::{ Queryable, Insertable, AsChangeset };
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug, Clone)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, AsChangeset, Debug, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
}
