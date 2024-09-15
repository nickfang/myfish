use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder, Result };
use env_logger::Env;
use serde::Serialize;
use diesel_migrations::{ EmbeddedMigrations, MigrationHarness };
use diesel::r2d2::ConnectionManager; // Add this line

mod schema;
mod models;
mod repositories;
mod handlers;
mod database;

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}

type DB = diesel::pg::Pg;
#[macro_use]
extern crate diesel_migrations;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(connection: &mut impl MigrationHarness<DB>) {
    let _ = connection.run_pending_migrations(MIGRATIONS);
}

async fn not_found_error() -> Result<HttpResponse> {
    Ok(
        HttpResponse::NotFound().json(Response {
            status: "error".to_string(),
            message: "Not Found".to_string(),
        })
    )
}

fn main() {
    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        let events_db = database::Database::new();
        run_migrations(&mut events_db.pool.get().unwrap());
        let app_data = web::Data::new(events_db);

        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
        HttpServer::new(move || {
            App::new()
                .app_data(app_data.clone())
                .configure(handlers::init_routes)
                // .service(health)
                .default_service(web::route().to(not_found_error))
                .wrap(actix_web::middleware::Logger::default())
        })
            .bind(("0.0.0.0", 8080))?
            .run().await
    }
}
