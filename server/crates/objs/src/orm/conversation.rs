use sea_orm::entity::prelude::*;

use super::types::{ConversationId, UserId};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "conversations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ConversationId,
    pub r#type: ConversationType,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub creator_id: Option<UserId>,
    pub created_at: DateTimeUtc,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum ConversationType {
    Private = 0,
    Group = 1,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::message::Entity",
        from = "Column::Id",
        to = "super::message::Column::ConversationId"
    )]
    Messages,
    #[sea_orm(
        has_many = "super::conversation_user::Entity",
        from = "Column::Id",
        to = "super::conversation_user::Column::CoversationId"
    )]
    ConversationUsers,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl Related<super::conversation_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationUsers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
