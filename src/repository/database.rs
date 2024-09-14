use diesel::prelude::*;
use diesel::r2d2::{ self, ConnectionManager };
use diesel::pg::PgConnection; // Add this line to import PgConnection
use dotenvy::dotenv;

use crate::models::users::{ User, NewUser };
use crate::schema::users::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

        Database { pool: result }
    }

    pub fn create_user(&self, user: NewUser) -> Result<User, diesel::result::Error> {
        diesel
            ::insert_into(users)
            .values(&user)
            .returning((id, username, email)) // Adjust fields as necessary
            .get_result(&mut self.pool.get().unwrap())
    }

    pub fn get_user_by_username(
        &self,
        username: &str
    ) -> Result<crate::models::User, diesel::result::Error> {
        use crate::models::schema::users::dsl::*;

        let conn = self.get_connection()?;
        users.filter(username.eq(username)).first(&conn)
    }

    pub fn get_user_by_email(
        &self,
        email: &str
    ) -> Result<crate::models::User, diesel::result::Error> {
        use crate::models::schema::users::dsl::*;

        let conn = self.get_connection()?;
        users.filter(email.eq(email)).first(&conn)
    }

    pub fn create_transaction(
        &self,
        date: chrono::NaiveDate,
        name: &str,
        amount: f64,
        transaction_type: &str,
        category: &str,
        description: &str,
        note: Option<&str>
    ) -> Result<usize, diesel::result::Error> {
        use crate::models::schema::transactions::dsl::*;

        let new_transaction = crate::models::Transactions {
            id: 0,
            date,
            name: name.to_string(),
            amount,
            transaction_type: transaction_type.to_string(),
            category: category.to_string(),
            description: description.to_string(),
            note: note.map(|s| s.to_string()),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        let conn = self.get_connection()?;
        diesel::insert_into(transactions).values(&new_transaction).execute(&conn)
    }

    pub fn get_transaction_by_id(
        &self,
        id: i32
    ) -> Result<crate::models::Transaction, diesel::result::Error> {
        use crate::models::schema::transactions::dsl::*;

        let conn = self.get_connection()?;
        transactions.find(id).first(&conn)
    }

    pub fn get_transactions_by_date(
        &self,
        date: chrono::NaiveDate
    ) -> Result<Vec<crate::models::Transaction>, diesel::result::Error> {
        use crate::models::schema::transactions::dsl::*;

        let conn = self.get_connection()?;
        transactions.filter(date.eq(date)).load(&conn)
    }

    pub fn get_transactions_by_name(
        &self,
        name: &str
    ) -> Result<Vec<crate::models::Transaction>, diesel::result::Error> {
        use crate::models::schema::transactions::dsl::*;

        let conn = self.get_connection()?;
        transactions.filter(name.eq(name)).load(&conn)
    }
}
