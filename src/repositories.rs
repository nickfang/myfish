use diesel::{ QueryDsl, QueryResult, ExpressionMethods };
use diesel_async::{ AsyncPgConnection, RunQueryDsl };

use crate::schema::{ users, transactions };
use crate::models::{ User, NewUser, Transaction, NewTransaction };

pub struct UserRepository;

impl UserRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await
    }
    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i32) -> QueryResult<Vec<User>> {
        users::table.limit(limit as i64).get_results(c).await
    }
    pub async fn create(c: &mut AsyncPgConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table).values(&new_user).get_result(c).await
    }
    pub async fn update(c: &mut AsyncPgConnection, user: User) -> QueryResult<User> {
        diesel
            ::update(users::table.find(user.id))
            .set((
                users::username.eq(user.username),
                users::email.eq(user.email),
                users::password.eq(user.password),
            ))
            .get_result(c).await
    }
    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c).await
    }
}

pub struct TransactionRepository;

impl TransactionRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Transaction> {
        transactions::table.find(id).get_result(c).await
    }
    pub async fn find_multiple(
        c: &mut AsyncPgConnection,
        limit: i32
    ) -> QueryResult<Vec<Transaction>> {
        transactions::table.limit(limit as i64).get_results(c).await
    }
    pub async fn create(
        c: &mut AsyncPgConnection,
        new_transaction: NewTransaction
    ) -> QueryResult<Transaction> {
        diesel::insert_into(transactions::table).values(&new_transaction).get_result(c).await
    }
    pub async fn update(
        c: &mut AsyncPgConnection,
        transaction: Transaction
    ) -> QueryResult<Transaction> {
        diesel
            ::update(transactions::table.find(transaction.id))
            .set((
                transactions::date.eq(transaction.date),
                transactions::name.eq(transaction.name),
                transactions::amount.eq(transaction.amount),
                transactions::transaction_type.eq(transaction.transaction_type),
                transactions::category.eq(transaction.category),
                transactions::description.eq(transaction.description),
                transactions::note.eq(transaction.note),
            ))
            .get_result(c).await
    }
    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(transactions::table.find(id)).execute(c).await
    }
}
