use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "todo_subscriptions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub todo_item_id: i32,
    pub user_id: Option<i32>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::todo_item::Entity",
        from = "Column::TodoItemId",
        to = "super::todo_item::Column::Id"
    )]
    TodoItem,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::todo_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TodoItem.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
