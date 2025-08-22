use crate::api::entities::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

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
    is_verified: bool,
    verification_token: Option<String>,
) -> Result<user::Model, sea_orm::DbErr> {
    let new_user = user::ActiveModel {
        email: Set(email.to_owned()),
        password: Set(password.to_owned()),
        username: Set(username.to_owned()),
        is_verified: Set(is_verified.to_owned()),
        verification_token: Set(verification_token),
        ..Default::default()
    };

    new_user.insert(db).await
}

pub async fn get_verif_token(
    db: &DatabaseConnection,
    token: &str,
) -> Result<Option<user::Model>, sea_orm::DbErr> {
    user::Entity::find()
        .filter(user::Column::VerificationToken.eq(token))
        .one(db)
        .await
}
