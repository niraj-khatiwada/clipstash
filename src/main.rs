use std::str::FromStr;

use actix_web::{self, middleware, web, App, HttpResponse, HttpServer, Responder};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use dotenvy::dotenv;

use lib::api::clip::{create_clip, delete_clip, get_clip, get_clips, update_clip};
use lib::{db::Database, domain::AppData};

#[actix_web::get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_port = dotenvy::var("SERVER_PORT").unwrap();

    std::env::set_var("RUST_LOG", "actix_web=debug, actix_server=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let database = Database::default().await;

    let app_data: AppData = AppData {
        db_pool: database.get_pool().clone(),
    };

    HttpServer::new(move || {
        let logger = middleware::Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(app_data.clone()))
            .service(ping)
            .service(get_clips)
            .service(get_clip)
            .service(create_clip)
            .service(update_clip)
            .service(delete_clip)
    })
    .bind(format!("127.0.0.1:{}", server_port))?
    .run()
    .await
}
