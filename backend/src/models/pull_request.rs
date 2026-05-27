use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pull_requests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub post_id: i32,
    pub user_email: String,
    pub content: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id"
    )]
    Post,
    #[sea_orm(has_many = "super::pull_request_comment::Entity")]
    PullRequestComment,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::pull_request_comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PullRequestComment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
