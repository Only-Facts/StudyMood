use crate::api::services::user;
use actix_web::{HttpResponse, Responder, get, web};
use sea_orm::DatabaseConnection;

#[get("/all")]
pub async fn show_users(conn: web::Data<DatabaseConnection>) -> impl Responder {
    match user::get_users(conn.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            eprintln!("Failed to fetch users: {err}");
            HttpResponse::InternalServerError().body("Error fetching users")
        }
    }
}

#[get("/{id}")]
pub async fn show_user(conn: web::Data<DatabaseConnection>, id: web::Path<u32>) -> impl Responder {
    let wid = id.into_inner();
    match user::get_user(conn.get_ref(), wid).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            eprintln!("Failed to fetch users: {err}");
            HttpResponse::InternalServerError().body("Error fetching users")
        }
    }
}
