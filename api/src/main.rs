mod database;
mod paths;

use actix_web::{App, HttpServer, web};
use crate::database::Database;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::logging::setup_logger()?;

    let port = std::env::var("INTERNAL_API_PORT").unwrap_or("8080".to_string());

    let db = match Database::new().await {
        Ok(db) => db,
        Err(e) => return Err(e),
    };
    let data = web::Data::new(db);

    log::info!("Starting server on 0.0.0.0:{port}...");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(paths::index)
            .service(paths::get_free)
            .service(paths::get_game)
            .service(paths::post_game)
            .service(paths::delete_game)
            .service(paths::get_posted)
            .service(paths::post_posted)
    })
        .bind(format!("0.0.0.0:{port}"))?
        .run()
        .await?;

    Ok(())
}
