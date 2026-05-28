# Learnings — TODO Handlers (T18)

## Auth Pattern for Mixed Admin/Visitor Endpoints
- The auth middleware (`middleware/auth.rs`) only blocks non-GET methods (POST/PUT/DELETE)
- GET requests always pass through without auth check
- For endpoints with different behavior per role (admin vs visitor), check `Authorization: Bearer <key>` header manually in the handler using `HeaderMap` extractor
- Compare token against `std::env::var("ADMIN_KEY")`

## SeaORM Patterns
- `ModelTrait` must be imported to call `.delete(db)` on a model instance
- `PaginatorTrait` must be imported to call `.count(db)` on a `Select`
- For counting related records efficiently, batch-fetch with `is_in(ids)` then count in-memory with `HashMap` — avoids N+1 queries

## Route Registration
- `Router::route()` returns `MethodRouter` which has `.delete()` method — no need to import `axum::routing::delete`
- Route groups use `nest("/api/v1/todos", todo_routes().with_state(state.clone()))`
- Handlers registered in `main.rs` router AND OpenAPI `#[openapi(paths(...))]` + `components(schemas(...))`

## File Pattern for New Endpoints
1. `handlers/{name}.rs` — request/response types (Deserialize/Serialize + utoipa::ToSchema) + handler fns with `#[utoipa::path]`
2. `routes/{name}.rs` — `fn {name}_routes() -> Router<Arc<AppState>>`
3. `handlers/mod.rs` — add `pub mod {name}`
4. `routes/mod.rs` — add `pub mod {name}`
5. `main.rs` — import route fn, add `.nest(...)`, register paths + schemas in OpenAPI, add tag
