use diesel::prelude::*;
use diesel::r2d2::{ self, ConnectionManager };
use dotenvy::dotenv;

use crate::models::{ User, NewUser };
use crate::schema::users::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let result = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

        Database { pool: result }
    }

    pub fn get_users(&self) -> Vec<User> {
        users.load::<User>(&mut self.pool.get().unwrap()).expect("Failed to get users.")
    }

    pub fn get_user(&self, find_id: i32) -> Option<User> {
        users.find(find_id).first::<User>(&mut self.pool.get().unwrap()).ok()
    }

    pub fn create_user(&self, user: NewUser) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users).values(&user).get_result(&mut self.pool.get().unwrap())
    }

    pub fn update_user(&self, user: User) -> Result<User, diesel::result::Error> {
        diesel
            ::update(users.filter(id.eq(user.id)))
            .set(&user)
            .get_result(&mut self.pool.get().unwrap())
    }
    pub fn delete_user(&self, user_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(users.find(user_id)).execute(&mut self.pool.get().unwrap())
    }
}
