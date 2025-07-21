use crate::api::services::{
    jwt::{get_token, verify_jwt},
    streaks,
};
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, web};
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
                Ok(streak) => HttpResponse::Ok().json(streak),
                Err(err) => {
                    eprintln!("Failed to fetch streak: {err}");
                    HttpResponse::InternalServerError().body("Error fetching streak")
                }
            }
        }
        Err(e) => {
            eprintln!("JWT Error: {e:?}");
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}

#[post("/")]
pub async fn init_streak(conn: web::Data<DatabaseConnection>, req: HttpRequest) -> impl Responder {
    let token = match get_token(req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };

    match verify_jwt(&token) {
        Ok(data) => {
            let user_id = data.claims.sub;
            match streaks::init_streak(conn.get_ref(), user_id).await {
                Ok(streak) => HttpResponse::Ok().json(streak),
                Err(err) => {
                    eprintln!("Failed to insert streak: {err}");
                    HttpResponse::InternalServerError().body("Error inserting streak")
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
                Ok(streak) => HttpResponse::Ok().json(streak),
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
