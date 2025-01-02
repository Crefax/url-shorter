use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;

mod models;
mod handlers;
mod utils;

use crate::handlers::{shorten_url, redirect_url, get_url_stats};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mongo_uri = env::var("MONGODB_URI")
        .expect("MONGODB_URI must be set");
    
    let client_options = ClientOptions::parse(&mongo_uri)
        .await
        .expect("Failed to parse MongoDB options");
    
    let client = Client::with_options(client_options)
        .expect("Failed to create MongoDB client");
    
    let db = client.database("url_shorter");
    let data = web::Data::new(db);

    println!("Server running at http://localhost:22822");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/api/shorten").route(web::post().to(shorten_url)))
            .service(web::resource("/api/stats/{short_code}").route(web::get().to(get_url_stats)))
            .service(web::resource("/{short_code}").route(web::get().to(redirect_url)))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:22822")?
    .run()
    .await
}
