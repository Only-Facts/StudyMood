use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use sea_orm::{DatabaseConnection, prelude::DateTimeUtc};
use serde::Deserialize;

use crate::api::services::{
    jwt::{get_token, verify_jwt},
    todo::{self, get_todos_by_uid},
};

#[derive(Debug, Deserialize)]
struct TodoRequest {
    title: String,
    description: String,
    dtime: DateTimeUtc,
    status: String,
}

#[derive(Debug, Deserialize)]
struct UpdateRequest {
    title: Option<String>,
    description: Option<String>,
    dtime: Option<DateTimeUtc>,
    status: Option<String>,
}

#[get("/all")]
pub async fn get_all_todos(
    conn: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> impl Responder {
    let token = match get_token(req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };
    match verify_jwt(&token) {
        Ok(_) => match todo::get_todos_all(&conn).await {
            Ok(todos) => HttpResponse::Ok().json(todos),
            Err(e) => {
                eprintln!("Failed to fetch todos: {e}");
                HttpResponse::InternalServerError().body("Error fetching todos")
            }
        },
        Err(e) => {
            eprintln!("JWT Error: {e}");
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}

#[get("/")]
pub async fn get_todos(conn: web::Data<DatabaseConnection>, req: HttpRequest) -> impl Responder {
    let token = match get_token(req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };
    match verify_jwt(&token) {
        Ok(data) => {
            let uid = data.claims.sub;
            match todo::get_todos_by_uid(&conn, uid).await {
                Ok(todos) => HttpResponse::Ok().json(todos),
                Err(e) => {
                    eprintln!("Failed to fetch todos: {e}");
                    HttpResponse::InternalServerError().body("Error fetching todos")
                }
            }
        }
        Err(e) => {
            eprintln!("JWT Error: {e}");
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}

#[post("/")]
pub async fn add_todo(
    conn: web::Data<DatabaseConnection>,
    req: HttpRequest,
    form: web::Json<TodoRequest>,
) -> impl Responder {
    let token = match get_token(req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };
    match verify_jwt(&token) {
        Ok(data) => {
            let uid = data.claims.sub;
            match todo::add_todo(
                &conn,
                &form.title,
                &form.description,
                form.dtime,
                uid,
                &form.status,
            )
            .await
            {
                Ok(todo) => HttpResponse::Ok().json(todo),
                Err(e) => {
                    eprintln!("Failed to insert todo: {e}");
                    HttpResponse::InternalServerError().body("Error inserting todo")
                }
            }
        }
        Err(e) => {
            eprintln!("JWT Error: {e}");
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}

#[delete("/{id}")]
pub async fn delete_todo(
    conn: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<u32>,
) -> impl Responder {
    let token = match get_token(req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };
    match verify_jwt(&token) {
        Ok(data) => {
            let id = path.into_inner();
            match get_todos_by_uid(&conn, data.claims.sub).await {
                Ok(todos) => {
                    for todo in todos {
                        if todo.id == id {
                            match todo::delete_todo(&conn, id).await {
                                Ok(_) => {
                                    return HttpResponse::Ok()
                                        .body("Todo (id: {id}) successfully deleted");
                                }
                                Err(e) => {
                                    eprintln!("Db Error: {e:?}");
                                    return HttpResponse::InternalServerError()
                                        .body("Error deleting todo");
                                }
                            }
                        }
                    }
                    HttpResponse::NotFound().body("Db Error: id: {id} was not found")
                }
                Err(e) => {
                    eprintln!("Failed to fetch todo: {e:?}");
                    HttpResponse::InternalServerError().body("Error fetching todos")
                }
            }
        }
        Err(e) => {
            eprintln!("JWT Error: {e:?}");
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}

#[put("/{id}")]
pub async fn update_todo(
    conn: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<u32>,
    form: web::Json<UpdateRequest>,
) -> impl Responder {
    let token = match get_token(req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };
    match verify_jwt(&token) {
        Ok(data) => {
            let id = path.into_inner();
            match get_todos_by_uid(&conn, data.claims.sub).await {
                Ok(todos) => {
                    for todo in todos {
                        if todo.id == id {
                            match todo::update_todo(
                                &conn,
                                id,
                                &form.title,
                                &form.description,
                                &form.dtime,
                                &form.status,
                            )
                            .await
                            {
                                Ok(option) => match option {
                                    Some(todo) => return HttpResponse::Ok().json(todo),
                                    None => return HttpResponse::Ok().body("Nothing was updated"),
                                },
                                Err(e) => {
                                    eprintln!("Db Error: {e:?}");
                                    return HttpResponse::InternalServerError()
                                        .body("Error updating todo");
                                }
                            }
                        }
                    }
                    HttpResponse::NotFound().body("Db Error: id: {id} was not found")
                }
                Err(e) => {
                    eprintln!("Failed to fetch todo: {e:?}");
                    HttpResponse::InternalServerError().body("Error fetching todos")
                }
            }
        }
        Err(e) => {
            eprintln!("JWT Error: {e:?}");
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}
