use sea_orm::entity::prelude::*;
use super::types::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: UserId,
    pub username: String,
    pub tagline: i32,
    pub password_hash: String,
    pub avatar: Option<String>,
    pub role: Role,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum Role {
    Human = 0,
    Bot = 1,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}