# Backend — Axum (Rust) Knowledge Base

## OVERVIEW

REST API server using Axum framework, SeaORM for PostgreSQL, lettre for SMTP emails. Single binary with optional DB connection.

## STRUCTURE

```
src/
├── main.rs          # Entrypoint: router assembly, OpenAPI, backup scheduler
├── lib.rs           # Public module re-exports (for integration tests)
├── config.rs        # Config struct from env vars (dotenvy)
├── state.rs         # AppState { db: Option<DatabaseConnection>, config: Config }
├── error.rs         # AppError enum (thiserror) → IntoResponse
├── handlers/        # Business logic (8 modules)
│   ├── posts.rs     # CRUD + #todo extraction + subscriber notifications
│   ├── pulls.rs     # PR-style interactions on posts
│   ├── subscribers.rs # Newsletter subscriber management
│   ├── images.rs    # Image upload to static/
│   ├── videos.rs    # Video upload (100MB limit)
│   ├── backup.rs    # DB backup/restore via pg_dump
│   └── export.rs    # JSON/CSV export
├── models/          # SeaORM entity definitions (6 active)
│   ├── post.rs      # posts table: id, title, slug, content, published, timestamps
│   ├── todo_item.rs # todo_items: auto-extracted from #todo in content
│   ├── subscriber.rs
│   ├── image.rs
│   ├── pull_request.rs
│   └── pull_request_comment.rs
├── routes/          # Route group definitions (nest with_state)
├── middleware/
│   └── auth.rs      # Bearer token auth (GET exempt, POST/PUT/DELETE require ADMIN_KEY)
├── migrations/      # SeaORM migration files (7)
└── tasks/
    ├── email.rs     # SMTP notifications via lettre
    └── backup.rs    # Scheduled pg_dump with retention
```

## WHERE TO LOOK

| Task | File | Pattern |
|------|------|---------|
| New endpoint | `handlers/{name}.rs` + `routes/{name}.rs` + `main.rs` | Handler fn → route registration → router |
| New model | `models/{name}.rs` + `models/mod.rs` + `migrations/` | Entity → mod pub → migration |
| New middleware | `middleware/{name}.rs` + `main.rs` | `.layer(axum::middleware::from_fn(...))` |
| Background task | `tasks/{name}.rs` | `tokio::spawn` in main.rs |

## CONVENTIONS

- **Handler signatures**: `async fn(State(state): State<Arc<AppState>>, ...) -> Result<Json<T>, AppError>`
- **utoipa annotations**: Every handler has `#[utoipa::path(...)]` for OpenAPI generation
- **Error propagation**: `?` operator with `AppError::from` conversions (DbErr, anyhow)
- **Response types**: Defined as structs with `#[derive(Serialize, utoipa::ToSchema)]`
- **Request types**: Defined as structs with `#[derive(Deserialize, utoipa::ToSchema)]`
- **Pagination**: `page` (1-based) + `per_page` (1-100), returns `{ items, total, page, per_page }`
- **Timestamps**: `chrono::DateTime<Utc>`, serialized as RFC 3339 strings
- **IDs**: `i32` (not UUID) for all primary keys
- **Slugs**: Auto-generated from title, stored alongside title

## ANTI-PATTERNS

- Do NOT use `unwrap()` in handlers — use `?` or `.ok_or(AppError::...)`
- Do NOT access `state.db` without `.as_ref().ok_or(...)` — DB is optional
- Do NOT forget utoipa annotations — breaks OpenAPI docs
- Do NOT use `dbg!()` or `println!()` — use `tracing::info!/error!/debug!`
- Do NOT create separate binaries — migrations run via `cargo run -- migrate` CLI arg

## COMMANDS

```bash
cargo run                    # Start server (needs DATABASE_URL, ADMIN_KEY)
cargo run -- migrate         # Run migrations only
cargo test                   # Unit + integration tests
cargo test -- --nocapture    # With println output
```

## NOTES

- `lib.rs` exists alongside `main.rs` — enables `use zyblog::...` in integration tests
- `AppState.db` is `Option<DatabaseConnection>` — app starts without DB
- Backup scheduler: spawned tokio task in main, runs every N hours
- Body limit: 105MB (for video uploads)
- Static files served from `backend/static/` via tower-http ServeDir
