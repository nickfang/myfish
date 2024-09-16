pub mod users;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(users::get_users);
    cfg.service(users::get_user);
    cfg.service(users::create_user);
    cfg.service(users::update_user);
    cfg.service(users::delete_user);
}
