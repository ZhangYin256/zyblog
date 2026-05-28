use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image: Option<String>,
    pub published: bool,
    pub author_id: Option<i32>,
    pub current_version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post_tag::Entity")]
    PostTag,
    #[sea_orm(has_many = "super::post_revision::Entity")]
    PostRevision,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AuthorId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::post_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostTag.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::post_tag::Relation::Post.def().rev())
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::post_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostRevision.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
