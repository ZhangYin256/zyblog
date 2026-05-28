
## Code Quality Review (2026-05-28)

### unwrap() in new handlers
- `auth.rs:92` — `encode(...).unwrap()` should use `?` or map_err (JWT encoding)
- `auth.rs:324` — `token_model.id.unwrap()` should use `.ok_or(AppError::...)`
- `media.rs:184` — `mime_to_extension().unwrap()` validated upstream, acceptable

### Dead code (17 warnings)
- `middleware/jwt.rs`: `dual_auth_middleware`, `role_rank`, `extract_bearer_token`, `ROLE_HIERARCHY` — never used
- `middleware/auth_guard.rs`: `require_role_middleware` — never used
- `utils/diff_engine.rs`: `generate_diff`, `apply_diff` — never used
- `utils/fragment_mapper.rs`: `map_selection_to_source` — never used
- `routes/*.rs`: 5 route functions defined but never wired into main router
- `models/image.rs`: `Model`, `Relation` — never constructed

### Frontend clean
- 0 `any` types in new composables and views
- `any` only in pre-existing Backup.vue, Import.vue, Publish.vue
