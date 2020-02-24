#![allow(unused_must_use)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_rt;
extern crate bcrypt;
extern crate derive_more;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate jsonwebtoken;
extern crate serde;
extern crate uuid;

mod api;
mod config;
mod constants;
mod entity;
mod error;
mod middleware;
mod schema;
mod services;
mod utils;

// use actix_redis::{Command, RedisActor};
use actix_service::Service;
use actix_web::{HttpServer, App};
use futures::FutureExt;
use std::{io, env};

#[actix_rt::main]
async fn main() -> io::Result<()>
{
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    // let redis_url = env::var("REDIS_URL").expect("REDIS_URL not found.");
    // let redis_serv = RedisActor::start(redis_url);

    let pool = config::db::migrate_and_config_db(&db_url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // .data(redis_serv.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(crate::middleware::auth_middleware::Authentication)
            .wrap_fn(|req, srv| {
                srv.call(req).map(|res| res)
            })
            .configure(config::app::config_services)
        })
    .bind(&app_url)?
    .run()
    .await
}
