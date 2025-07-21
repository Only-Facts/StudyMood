use crate::api::services::{
    jwt::{get_token, verify_jwt},
    streaks,
};
use actix_web::{HttpRequest, HttpResponse, Responder, get, put, web};
use sea_orm::DatabaseConnection;

#[get("/")]
pub async fn get_streak(conn: web::Data<DatabaseConnection>, req: HttpRequest) -> impl Responder {
    let token = match get_token(req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };

    match verify_jwt(&token) {
        Ok(data) => {
            let user_id = data.claims.sub;
            match streaks::get_streak(conn.get_ref(), user_id).await {
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

#[put("/")]
pub async fn update_streak(
    conn: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> impl Responder {
    let token = match get_token(req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };

    match verify_jwt(&token) {
        Ok(data) => {
            let user_id = data.claims.sub;
            match streaks::update_streak(conn.get_ref(), user_id).await {
                Ok(()) => HttpResponse::Ok().body("Updated Streak"),
                Err(e) => {
                    eprintln!("Failed to update streak: {e}");
                    HttpResponse::InternalServerError().body("Error updating streak")
                }
            }
        }
        Err(e) => {
            eprintln!("JWT Error: {e:?}");
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}
