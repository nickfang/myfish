use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder, Result };
use env_logger::Env;
use serde::Serialize;
use diesel_migrations::{ EmbeddedMigrations, MigrationHarness };

#[macro_use]
extern crate diesel_migrations;

mod schema;
mod models;
mod repositories;
mod handlers;

fn main() {
    println!("Hello, world!");
}
