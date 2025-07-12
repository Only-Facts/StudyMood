use crate::api::services::{jwt::verify_jwt, user};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
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

#[get("/")]
pub async fn show_user(conn: web::Data<DatabaseConnection>, req: HttpRequest) -> impl Responder {
    let token = match req.headers().get("Authorization") {
        Some(header) => {
            let bearer = header.to_str().unwrap_or_default();
            if bearer.starts_with("Bearer ") {
                Some(bearer.trim_start_matches("Bearer ").to_string())
            } else {
                None
            }
        }
        None => None,
    };
    let token = match token {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };

    match verify_jwt(&token) {
        Ok(data) => {
            let user_id = data.claims.sub;
            match user::get_user(conn.get_ref(), user_id).await {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(err) => {
                    eprintln!("Failed to fetch user: {err}");
                    HttpResponse::InternalServerError().body("Error fetching user")
                }
            }
        }
        Err(e) => {
            eprintln!("JWT Error: {e:?}");
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}
