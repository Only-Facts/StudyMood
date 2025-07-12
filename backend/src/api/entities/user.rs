use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub email: String,
    #[sea_orm(column_name = "pass")]
    pub password: String,
    pub created_at: Option<DateTimeUtc>,
    #[sea_orm(column_name = "fname")]
    pub first_name: String,
    #[sea_orm(column_name = "lname")]
    pub last_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("Relation Error: No relations for user entity.")
    }
}
impl ActiveModelBehavior for ActiveModel {}
