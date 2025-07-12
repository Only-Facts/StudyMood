mod db;
pub mod entities;
mod routes;
pub mod services;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{self, Data},
};
use routes::{auth, music, user};
use sea_orm::DatabaseConnection;
use services::loader;
use std::{path::PathBuf, sync::Mutex};

const MUSIC_DIRECTORY_ENV_VAR: &str = "MUSIC_DIR";

#[get("/")]
async fn default(conn: web::Data<DatabaseConnection>) -> impl Responder {
    if conn.ping().await.is_ok() {
        HttpResponse::Ok().body("Database connection is alive ! :D")
    } else {
        HttpResponse::InternalServerError().body("Failed to ping DB. :(")
    }
}

async fn load_musics() -> Result<Data<loader::AppState>, std::io::Error> {
    let music_dir_str =
        std::env::var(MUSIC_DIRECTORY_ENV_VAR).unwrap_or_else(|_| String::from("./music"));
    let music_dir = PathBuf::from(music_dir_str);
    let tracks_map = match loader::load_music_files(&music_dir).await {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error loading music files: {e}");
            return Err(std::io::Error::other(e));
        }
    };
    Ok(web::Data::new(loader::AppState::new(
        music_dir.clone(),
        Mutex::new(tracks_map),
    )))
}

#[allow(clippy::manual_strip, clippy::io_other_error)]
pub async fn api() -> std::io::Result<()> {
    let app_state = load_musics()
        .await
        .expect("AppState Error: Failed to load musics");
    let conn = web::Data::new(db::init_db().await);
    println!("[✅] DB Connection Success !");
    HttpServer::new(move || {
        App::new()
            .app_data(conn.clone())
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
                web::scope("/users")
                    .wrap(
                        Cors::permissive()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                            .max_age(3600),
                    )
                    .app_data(conn.clone())
                    .service(user::show_users)
                    .service(user::show_user),
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
                    .service(auth::register_user)
                    .service(auth::login_user),
            )
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
