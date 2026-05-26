# ZYBlog Test Implementation Learnings

## Backend Testing (Rust/Axum)

### Architecture
- Binary crate needed `lib.rs` to expose modules for integration tests
- Private functions tested via inline `#[cfg(test)]` modules
- Integration tests use `tower::util::ServiceExt` for HTTP testing

### Key Patterns
- `axum::Router::oneshot()` for testing HTTP handlers without starting a server
- `AppState` with `db: None` to test error handling paths
- Auth middleware tests accept both 401 and 500 (env var race conditions)
- `serde_json` deserialization tests for request/response types

### Test Coverage
- 4 unit tests (inline): extract_todos, generate_slug
- 6 handler tests: request/response deserialization
- 8 error tests: AppError variants and status codes
- 5 config tests: Config::from_env behavior
- 11 integration tests: HTTP routing, auth middleware, error responses
- 3 image tests: upload validation

## Frontend Testing (Vue 3/Vitest)

### Setup
- `vitest` + `happy-dom` for DOM environment
- `@vue/test-utils` for component mounting
- `@pinia/testing` for Pinia store mocking
- `@vitest/coverage-v8` for code coverage

### Key Patterns
- `vi.mock()` for module mocking (vue-router, naive-ui, composables)
- `createTestingPinia()` for isolated store state
- Component tests focus on rendering and user interactions

### Coverage Results
- Overall: ~28% statements
- `usePosts.ts`: 86% statements
- `TodoSubscribe.vue`: 55% statements

## Gotchas
1. Env var tests are racy in Rust (parallel test execution)
2. Private functions can't be imported in integration tests
3. `@vue/server-renderer` required peer dep for @vue/test-utils
4. `@pinia/testing` required for component tests using Pinia

## Scope Fidelity Check Learnings (F4)

### Architecture Observations
- All 17 tasks implemented in single working tree (only 1 git commit: init)
- Frontend uses inline composables rather than separate files (e.g., auto-save in Publish.vue)
- Backend tasks are tightly coupled via main.rs router configuration
- Image upload routes placed in public_routes (no auth) vs acceptance criteria

### Cross-Task Coupling Patterns
- Task 6 (Post CRUD) → Task 9 (Email): update_post calls notify_subscribers
- Task 6 (Post CRUD) → Task 8 (Auth): routes nested under protected_routes
- These are acceptable architectural patterns but violate strict task isolation

### Missing Deliverables
- frontend/Dockerfile: referenced by docker-compose.yml but never created
- /posts route: menu links to /posts but router only has /posts/:id
- All implementation code uncommitted beyond initial git init

## Scope Fidelity Issues Summary (F4)

### HIGH: Frontend Dockerfile Missing (Task 5)
docker-compose.yml references frontend/Dockerfile but it was never created.

### HIGH: All Code Uncommitted (Global)
Only 1 git commit (init). All Task 2-17 code is uncommitted working tree.

### MEDIUM: Image Upload No Auth (Task 7)
Image routes placed in public_routes despite spec requiring Bearer token auth.

### MEDIUM: Menu Route Bug (Task 12)
"文章列表" links to /posts but router only has /posts/:id, not /posts.

### MEDIUM: Cross-Task Email Logic (Task 6/9)
update_post handler calls notify_subscribers_on_update (Task 9 boundary).

### LOW: useAutoSave.ts Not Separate File (Task 11)
Auto-save inline in Publish.vue instead of separate composable.

### LOW: HelloWorld.vue Leftover (Task 3)
Vue scaffolding dead code not cleaned up.
