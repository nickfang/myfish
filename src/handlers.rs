use actix_web::{ web, get, post, delete, put, HttpResponse };
use crate::models::{ User, NewUser };
use crate::database::Database;

#[get("/users")]
pub async fn get_users(db: web::Data<Database>) -> HttpResponse {
    let users = db.get_users();
    HttpResponse::Ok().json(users)
}

#[get("/users/{id}")]
pub async fn get_user(db: web::Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let user = db.get_user(id.into_inner()).unwrap();
    HttpResponse::Ok().json(user)
}

#[post("/users")]
pub async fn create_user(
    db: web::Data<Database>,
    new_user: web::Json<NewUser>
) -> Result<HttpResponse, actix_web::Error> {
    let user = db.create_user(new_user.into_inner());
    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(db: web::Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let user = db.delete_user(id.into_inner()).unwrap();
    HttpResponse::Ok().json(user)
}

#[put("/users/{id}")]
pub async fn update_user(db: web::Data<Database>, user: web::Json<User>) -> HttpResponse {
    let user = db.update_user(user.into_inner()).unwrap();
    HttpResponse::Ok().json(user)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(get_user);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}
