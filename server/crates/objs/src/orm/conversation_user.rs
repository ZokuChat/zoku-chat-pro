use sea_orm::entity::prelude::*;

use super::types::{ConversationId, UserId};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "conversation_users")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub coversation_id: ConversationId,
  #[sea_orm(primary_key)]
  pub user_id: UserId,
  pub joined_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::conversation::Entity",
    from = "Column::CoversationId",
    to = "super::conversation::Column::Id"
  )]
  Conversation,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id"
  )]
  User,
}

impl Related<super::conversation::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Conversation.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}