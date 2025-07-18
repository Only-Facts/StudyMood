use crate::api::entities::todo;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, prelude::DateTimeUtc,
};

pub async fn get_todos_all(db: &DatabaseConnection) -> Result<Vec<todo::Model>, sea_orm::DbErr> {
    todo::Entity::find().all(db).await
}

pub async fn get_todos_by_uid(
    db: &DatabaseConnection,
    uid: u32,
) -> Result<Vec<todo::Model>, sea_orm::DbErr> {
    todo::Entity::find()
        .filter(todo::Column::Uid.eq(uid))
        .all(db)
        .await
}

pub async fn add_todo(
    db: &DatabaseConnection,
    title: &str,
    descripton: &str,
    dtime: DateTimeUtc,
    uid: u32,
    status: &str,
) -> Result<todo::Model, sea_orm::DbErr> {
    let new_todo = todo::ActiveModel {
        title: Set(title.to_owned()),
        descr: Set(descripton.to_owned()),
        dtime: Set(dtime.to_owned()),
        uid: Set(uid.to_owned()),
        status: Set(status.to_owned()),
        ..Default::default()
    };
    new_todo.insert(db).await
}

pub async fn delete_todo(
    db: &DatabaseConnection,
    id: u32,
) -> Result<sea_orm::DeleteResult, sea_orm::DbErr> {
    todo::Entity::delete_by_id(id).exec(db).await
}

pub async fn update_todo(
    db: &DatabaseConnection,
    id: u32,
    title: &Option<String>,
    description: &Option<String>,
    dtime: &Option<DateTimeUtc>,
    status: &Option<String>,
) -> Result<Option<todo::Model>, sea_orm::DbErr> {
    if let Some(model) = todo::Entity::find_by_id(id).one(db).await? {
        let mut active_model = model.into_active_model();

        if let Some(title) = title {
            active_model.title = Set(title.to_string());
        }
        if let Some(description) = description {
            active_model.descr = Set(description.to_string());
        }
        if let Some(dtime) = dtime {
            active_model.dtime = Set(dtime.to_utc());
        }
        if let Some(status) = status {
            active_model.status = Set(status.to_string());
        }

        Ok(Some(active_model.update(db).await?))
    } else {
        Ok(None)
    }
}
