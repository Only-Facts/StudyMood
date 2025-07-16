use crate::api::entities::user as user_entity;
use crate::api::services::{jwt::create_jwt, user};
use actix_web::{HttpResponse, Responder, post, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
    fname: String,
    lname: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register_user(
    conn: web::Data<DatabaseConnection>,
    form: web::Json<RegisterRequest>,
) -> impl Responder {
    let hashed = match hash(&form.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    match user::add_user(
        conn.get_ref(),
        &form.email.clone(),
        &hashed,
        &form.fname.clone(),
        &form.lname.clone(),
    )
    .await
    {
        Ok(usr) => {
            let token = match create_jwt(usr.id) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("JWT Error: {e:?}");
                    return HttpResponse::InternalServerError().body("Could not register user");
                }
            };
            HttpResponse::Ok().json(serde_json::json!({
                "token": token
            }))
        }
        Err(e) => {
            eprintln!("Error: inserting user: {e:?}");
            HttpResponse::InternalServerError().body("Could not register user")
        }
    }
}

#[post("/login")]
pub async fn login_user(
    conn: web::Data<DatabaseConnection>,
    form: web::Json<LoginRequest>,
) -> impl Responder {
    let found_user = user_entity::Entity::find()
        .filter(user_entity::Column::Email.eq(form.email.clone()))
        .one(conn.get_ref())
        .await;
    let user_model = match found_user {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(e) => {
            eprintln!("DB Error: {e:?}");
            return HttpResponse::InternalServerError().body("Databse error");
        }
    };

    let is_valid = verify(&form.password, &user_model.password);
    if is_valid.unwrap_or(false) {
        match create_jwt(user_model.id) {
            Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
            Err(e) => {
                eprintln!("JWT Error: {e:?}");
                HttpResponse::InternalServerError().body("Token creation failed")
            }
        }
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}
