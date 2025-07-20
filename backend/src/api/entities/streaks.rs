use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "streaks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub current_streak: i32,
    pub longest_streak: i32,
    pub last_active: NaiveDate,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
