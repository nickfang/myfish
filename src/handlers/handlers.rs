// use actix_web::{ web, get, post, delete, put, HttpResponse };
// use crate::repositories::{ UserRepository, TransactionRepository };
// use crate::{ models::users::{ User, NewUser } };

// #[get("/users")]
// pub async fn get_users(db: web::Data<Database>) -> HttpResponse {
//     let users = db.get_users().unwrap();
//     HttpResponse::Ok().json(users)
// }

// #[get("/users/{id}")]
// pub async fn get_user(db: web::Data<Database>, id: web::Path<i32>) -> HttpResponse {
//     let user = db.get_user(id.into_inner()).unwrap();
//     HttpResponse::Ok().json(user)
// }

// #[post("/users")]
// pub async fn create_user(db: web::Data<Database>, user: web::Json<NewUser>) -> HttpResponse {
//     let user = db.create_user(user.into_inner()).unwrap();
//     HttpResponse::Ok().json(user)
// }

// pub fn init_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(get_users);
//     cfg.service(get_user);
//     cfg.service(create_user);
// }
