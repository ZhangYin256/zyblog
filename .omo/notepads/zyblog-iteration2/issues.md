## Issues

## Task 4: Model Cleanup (2026-05-28)

**Issue**: Duplicate model files existed alongside correct ones:
- `pull.rs` (old, had `title`+`updated_at` fields not in DB) alongside `pull_request.rs` (correct)
- `pull_comment.rs` (old, had `user_email`+`pull_id` not matching DB) alongside `pull_request_comment.rs` (correct)

**Resolution**: Deleted `pull.rs` and `pull_comment.rs`. Neither was declared in `mod.rs` nor referenced by any handler code.

**Verification**: `cargo build` passes. Only pre-existing dead_code warnings remain (routes/images unused in lib context).
