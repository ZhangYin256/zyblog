use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pull_request_comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pull_request_id: i32,
    pub content: String,
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
}

impl Related<super::pull_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PullRequest.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
