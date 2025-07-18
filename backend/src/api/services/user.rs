use crate::api::entities::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub async fn get_users(db: &DatabaseConnection) -> Result<Vec<user::Model>, sea_orm::DbErr> {
    user::Entity::find().all(db).await
}

pub async fn get_user(
    db: &DatabaseConnection,
    id: u32,
) -> Result<Option<user::Model>, sea_orm::DbErr> {
    user::Entity::find_by_id(id).one(db).await
}

pub async fn add_user(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
    username: &str,
) -> Result<user::Model, sea_orm::DbErr> {
    let new_user = user::ActiveModel {
        email: Set(email.to_owned()),
        password: Set(password.to_owned()),
        username: Set(username.to_owned()),
        ..Default::default()
    };

    new_user.insert(db).await
}
