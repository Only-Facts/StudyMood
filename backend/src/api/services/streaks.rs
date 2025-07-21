use crate::api::entities::{streaks, user};
use chrono::{Duration, Local};
use sea_orm::{DatabaseConnection, entity::*};

pub async fn get_streak(
    db: &DatabaseConnection,
    user_id: u32,
) -> Result<Option<streaks::Model>, sea_orm::DbErr> {
    streaks::Entity::find_by_id(user_id).one(db).await
}

pub async fn init_streak(
    db: &DatabaseConnection,
    user_id: u32,
) -> Result<Option<streaks::Model>, sea_orm::DbErr> {
    let today = Local::now().date_naive();
    let new_streak = streaks::ActiveModel {
        user_id: Set(user_id.to_owned()),
        current_streak: Set(1.to_owned()),
        longest_streak: Set(1.to_owned()),
        last_active: Set(today.to_owned()),
    };

    if let Err(e) = new_streak.insert(db).await {
        if e.to_string() != "None of the records are inserted" {
            return Err(e);
        }
    }
    get_streak(db, user_id).await
}

pub async fn update_streak(
    db: &DatabaseConnection,
    user_id: u32,
) -> Result<Option<streaks::Model>, sea_orm::DbErr> {
    let user_exists = user::Entity::find_by_id(user_id).one(db).await?.is_some();

    if !user_exists {
        return Err(sea_orm::DbErr::Custom(format!(
            "User id: {user_id} does not exist"
        )));
    }
    let today = Local::now().date_naive();

    if let Some(streak) = streaks::Entity::find_by_id(user_id).one(db).await? {
        let last_active = streak.last_active;
        let current_streak = streak.current_streak;
        let longest_streak = streak.longest_streak;

        if last_active == today {
            return get_streak(db, user_id).await;
        }

        let new_streak = if last_active == today - Duration::days(1) {
            current_streak + 1
        } else {
            1
        };

        let new_longest = longest_streak.max(new_streak);

        let mut active_model: streaks::ActiveModel = streak.into();
        active_model.current_streak = Set(new_streak);
        active_model.last_active = Set(today);
        active_model.longest_streak = Set(new_longest);

        active_model.update(db).await?;
    } else {
        let new_streak = streaks::ActiveModel {
            user_id: Set(user_id),
            current_streak: Set(1),
            longest_streak: Set(1),
            last_active: Set(today),
        };

        new_streak.insert(db).await?;
    }

    get_streak(db, user_id).await
}
