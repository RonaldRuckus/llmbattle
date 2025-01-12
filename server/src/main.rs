use actix_web::{web, App, HttpServer};
use chrono::Local;
use dotenv::dotenv;
use handlers::{handle_create, handle_join, handle_poll};
use log::LevelFilter;
use std::{env, fs, path::Path, sync::Mutex};
use rusqlite::Connection;
pub mod models;
pub mod db;
mod handlers;
use db::initialize_database;

fn configure_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = Path::new("./logs");
    if !log_dir.exists() {
        fs::create_dir_all(log_dir)?;
    }

    let log_file_path = format!("{}/{}.log", log_dir.display(), Local::now().format("%Y-%m-%d_%H-%M-%S"));

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info) // Log all INFO-level messages or higher
        .chain(std::io::stderr()) // Log to stderr
        .chain(fern::log_file(log_file_path)?) // Log to the file
        .apply()?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    configure_logging().expect("Failed to configure logging");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let schema_file_path = env::var("SCHEMA_FILE_PATH").expect("SCHEMA_FILE_PATH must be set in .env");

    log::info!("Starting server with logging enabled");

    let conn = Connection::open(database_url).expect("Failed to connect to the database.");
    initialize_database(&conn, &schema_file_path);

    let data = web::Data::new(Mutex::new(conn));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/create", web::post().to(handle_create))
            .route("/{game_id}/join", web::post().to(handle_join))
            .route("/{game_id}/poll", web::get().to(handle_poll))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
