use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pr_comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pull_request_id: i32,
    pub fragment_index: Option<i32>,
    pub line: Option<i32>,
    pub content: String,
    pub user_id: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::pull_request::Entity",
        from = "Column::PullRequestId",
        to = "super::pull_request::Column::Id"
    )]
    PullRequest,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::pull_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PullRequest.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
