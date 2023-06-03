#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
use std::sync::Arc;
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use env_logger;

pub mod schema;
pub mod models;
pub mod routes;
mod errors;
mod vars;

use actix_web::{
    HttpServer,
    App,
    middleware::Compress,
    web,
    http,
};

use actix_files::Files;
use crate::routes::routes;

#[macro_use]
mod utils;
#[macro_use]
mod views;

#[derive(Clone)]
pub struct AppState {
    key: Arc<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_state = AppState {
        key: Arc::new(env::var("KEY").unwrap()),
    };

    HttpServer::new(move || {
        let _files = Files::new("/media", "media/").show_files_listing();

        App::new()
            .wrap(Compress::default())
            .app_data(web::Data::new(app_state.to_owned()))
            .app_data(web::JsonConfig::default().limit(4096))
            .service(_files)
            .configure(routes)
    })

    //.bind("176.99.2.88:9090")?   // порт для разработки
    .bind("151.248.120.218:9091")? // порт для автоматической доставки
    .run()
    .await
}
