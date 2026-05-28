## Learnings

## Tags Migration & Entity (Task 8)

### SeaORM Composite Primary Key
- Use `#[sea_orm(primary_key, auto_increment = false)]` on multiple fields for composite PK
- In migration, use `.primary_key(Index::create().col(...).col(...))` on the `TableCreateStatement`

### SeaORM Many-to-Many via Join Entity
- Intermediate entity (post_tag) has `belongs_to` relations to both sides
- Each side entity has `has_many` to intermediate + `Related<ViaEntity>` with `fn via()` for many-to-many
- `via()` returns `Some(relation.def().rev())` pointing to the intermediate's relation to the other side

### Migration Pattern
- Migration Iden enums need to reference foreign tables (e.g., `Posts::Table, Posts::Id`) for FK definitions
- Index on foreign key column recommended for performance (e.g., `idx_post_tags_tag_id`)

## Task 9: Tags API Handlers — 2026-05-28

### Patterns Confirmed
- Handler pattern: `async fn(State(state): State<Arc<AppState>>, ...) -> Result<Json<T>, AppError>`
- DB access: `state.db.as_ref().ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?`
- utoipa: every handler needs `#[utoipa::path(...)]` with `tag = "tags"`
- Route nesting: `Router::new().route("/", get(fn).post(fn)).route("/:id", delete(fn))`
- Registration: `.nest("/api/v1/tags", tag_routes().with_state(state.clone()))`
- For post-sub-resources: register directly with `.route("/api/v1/posts/:id/tags", ...)` instead of nesting

### Gotchas
- `delete` must be explicitly imported: `use axum::routing::{delete, get, post, put}`
- When a model's `delete()` method consumes `self`, save fields before calling `.delete(db)`
- Parallel task coordination: main.rs referenced comments routes before mod.rs declared them → had to add `pub mod comments;` to both handlers/mod.rs and routes/mod.rs
- Models (tag.rs, post_tag.rs) and migrations (008, 009) already existed from Task 6
- post_tag has composite primary key (post_id, tag_id) — use `auto_increment = false` on both fields

### SeaORM Composite Key Pattern
```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "post_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub post_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i32,
}
```
Find by composite key: `Entity::find_by_id((post_id, tag_id)).one(db).await?`
