use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pull_request_comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pull_id: i32,
    pub user_email: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::pull::Entity",
        from = "Column::PullId",
        to = "super::pull::Column::Id"
    )]
    Pull,
}

impl Related<super::pull::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pull.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
