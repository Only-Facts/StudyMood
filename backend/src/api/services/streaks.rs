use crate::api::entities::streaks;
use chrono::{Duration, Local};
use sea_orm::{DatabaseConnection, entity::*};

pub async fn get_streak(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Option<streaks::Model>, sea_orm::DbErr> {
    streaks::Entity::find_by_id(user_id).one(db).await
}

pub async fn update_streak(db: &DatabaseConnection, user_id: i32) -> Result<(), sea_orm::DbErr> {
    let today = Local::now().date_naive();

    if let Some(streak) = streaks::Entity::find_by_id(user_id).one(db).await? {
        let last_active = streak.last_active;
        let current_streak = streak.current_streak;
        let longest_streak = streak.longest_streak;

        if last_active == today {
            return Ok(());
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

    Ok(())
}
