use crate::api::entities::user;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set};

pub async fn get_users(db: &DbConn) -> Result<Vec<user::Model>, sea_orm::DbErr> {
    user::Entity::find().all(db).await
}

pub async fn get_user(db: &DbConn, id: u32) -> Result<Option<user::Model>, sea_orm::DbErr> {
    user::Entity::find_by_id(id).one(db).await
}

pub async fn add_user(
    db: &DbConn,
    email: &str,
    password: &str,
    first_name: &str,
    last_name: &str,
) -> Result<user::Model, sea_orm::DbErr> {
    let new_user = user::ActiveModel {
        email: Set(email.to_owned()),
        password: Set(password.to_owned()),
        first_name: Set(first_name.to_owned()),
        last_name: Set(last_name.to_owned()),
        ..Default::default()
    };

    new_user.insert(db).await
}
