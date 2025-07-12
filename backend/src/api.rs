mod db;
pub mod entities {
    pub mod todo;
    pub mod user;
}
mod services;
use services::{auth, music};

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use sea_orm::DbConn;
use std::{path::PathBuf, sync::Mutex};

const MUSIC_DIRECTORY_ENV_VAR: &str = "MUSIC_DIR";

#[get("/")]
async fn default(db: web::Data<DbConn>) -> impl Responder {
    if db.ping().await.is_ok() {
        HttpResponse::Ok().body("✅ Database connection is alive!")
    } else {
        HttpResponse::InternalServerError().body("❌ Failed to ping DB.")
    }
}

#[allow(clippy::manual_strip, clippy::io_other_error)]
pub async fn api() -> std::io::Result<()> {
    let music_dir_str =
        std::env::var(MUSIC_DIRECTORY_ENV_VAR).unwrap_or_else(|_| String::from("./music"));
    let music_dir = PathBuf::from(music_dir_str);
    let tracks_map = match music::load_music_files(&music_dir).await {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error loading music files: {e}");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };
    let app_state = web::Data::new(music::AppState::new(
        music_dir.clone(),
        Mutex::new(tracks_map),
    ));
    let conn = web::Data::new(db::init_db().await);
    println!("[✅] DB Connection Success !");
    HttpServer::new(move || {
        App::new()
            .service(default)
            .service(
                web::scope("/music")
                    .wrap(
                        Cors::permissive()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                            .max_age(3600),
                    )
                    .app_data(app_state.clone())
                    .service(music::list)
                    .service(music::stream),
            )
            .service(
                web::scope("/auth")
                    .wrap(
                        Cors::permissive()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                            .max_age(3600),
                    )
                    .app_data(conn.clone())
                    .service(auth::show_users)
                    .service(auth::show_user),
            )
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
