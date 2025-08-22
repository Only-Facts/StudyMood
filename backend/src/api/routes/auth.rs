use crate::api::entities::user as user_entity;
use crate::api::services::{email, jwt::create_jwt, token, user};
use actix_web::{HttpResponse, Responder, get, post, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
    username: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct VerificationParams {
    token: String,
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

    let token = token::generate_token(32);
    match user::add_user(
        conn.get_ref(),
        &form.email.clone(),
        &hashed,
        &form.username.clone(),
        false,
        Some(token.clone()),
    )
    .await
    {
        Ok(_) => match email::send_verification_email(&form.email.clone(), &token).await {
            Ok(_) => HttpResponse::Created()
                .body("Signup successful! Please check your email to verify your account."),
            Err(e) => {
                eprintln!("Failed to send verification email: {e:?}");
                HttpResponse::InternalServerError().body("Could not send verification email.")
            }
        },
        Err(e) => {
            eprintln!("Error: inserting user: {e:?}");
            HttpResponse::Conflict().body("User already exists")
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

#[get("/verify")]
pub async fn verify_email(
    conn: web::Data<DatabaseConnection>,
    query: web::Query<VerificationParams>,
) -> impl Responder {
    match user::get_verif_token(conn.get_ref(), &(query.token)).await {
        Ok(opt) => match opt {
            Some(usr) => {
                let token = match create_jwt(usr.id) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("JWT Error: {e:?}");
                        return HttpResponse::InternalServerError().body("Could not verify user");
                    }
                };

                let mut active_model = usr.into_active_model();
                active_model.is_verified = Set(true);
                active_model.verification_token = Set(None);

                HttpResponse::Found().json(serde_json::json!({
                    "token": token,
                    "Location": "/profile"
                }))
            }
            None => HttpResponse::NotFound().body("User not found"),
        },
        Err(e) => {
            eprintln!("Error verifying user: {e:?}");
            HttpResponse::Conflict().body("Could not verify user")
        }
    }
}
