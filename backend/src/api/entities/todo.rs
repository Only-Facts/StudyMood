use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub title: String,
    #[sea_orm(column_name = "description")]
    pub descr: String,
    pub created_at: Option<DateTimeUtc>,
    pub dtime: DateTimeUtc,
    #[sea_orm(column_name = "user_id")]
    pub uid: u32,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Uid)
                .to(super::user::Column::Id)
                .into(),
        }
    }
}
impl ActiveModelBehavior for ActiveModel {}
