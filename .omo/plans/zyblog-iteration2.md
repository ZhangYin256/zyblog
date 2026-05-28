# ZYBlog Iteration 2 — Stability + Features + Quality

## TL;DR

> **Quick Summary**: Fix Docker infrastructure, add 5 features (tags, search, comments, theme toggle, rich markdown), then improve code quality with e2e tests.
>
> **Deliverables**:
> - Working `docker-compose build` for both services
> - Tags/categories system with many-to-many post relationship
> - Basic search (ILIKE on title + content)
> - Moderated reader comments (admin approval)
> - Dark/light theme toggle
> - Rich markdown rendering (syntax highlighting, LaTeX, mermaid)
> - Playwright e2e tests for critical paths
>
> **Estimated Effort**: Large
> **Parallel Execution**: YES — 4 waves
> **Critical Path**: Docker fix → DB models → Backend features → Frontend features → Tests

---

## Context

### Original Request
User requested comprehensive iteration covering DevOps stability, new features, code quality, and production readiness (Docker fix only).

### Interview Summary
**Key Discussions**:
- **Comments**: Moderated — admin approval before public display
- **Search**: Basic ILIKE on title + content (no full-text search)
- **Markdown**: Rich rendering with syntax highlighting, LaTeX math, mermaid diagrams
- **Deployment**: Docker fix only (no production deploy this iteration)
- **Priority**: Stability → Features → Quality

**Research Findings**:
- Frontend Dockerfile missing — `docker-compose build` fails
- `pull.rs` / `pull_comment.rs` exist but not declared in `models/mod.rs`
- `HelloWorld.vue` is unused scaffold leftover
- `run.txt` contains hardcoded admin key
- No `.dockerignore` files exist

---

## Work Objectives

### Core Objective
Make the blog fully functional via Docker, add 5 user-facing features, and establish e2e test coverage.

### Concrete Deliverables
- `frontend/Dockerfile` — Multi-stage Node build
- `.dockerignore` files — Root + backend
- `comments` table + API + moderated approval flow
- `tags` + `post_tags` tables + API
- Search endpoint with ILIKE queries
- Theme toggle (dark/light) with CSS variables
- Rich markdown rendering (markdown-it + highlight.js + KaTeX + mermaid)
- Playwright e2e test suite

### Definition of Done
- [ ] `docker-compose build` succeeds for both services
- [ ] `docker-compose up -d` starts all 3 services
- [ ] Tags CRUD works via API
- [ ] Search returns filtered results
- [ ] Comments can be submitted, approved, and displayed
- [ ] Theme toggle persists across page reloads
- [ ] Markdown renders with code highlighting, math, and diagrams
- [ ] Playwright tests pass for critical paths

### Must Have
- Working Docker setup
- All 5 features functional
- E2e tests for new features

### Must NOT Have (Guardrails)
- Do NOT add user authentication (keep single-admin model)
- Do NOT add CI/CD pipeline (future iteration)
- Do NOT deploy to production (Docker fix only)
- Do NOT refactor existing working code unless broken
- Do NOT add SSR/SSG — keep SPA
- Do NOT use full-text search (ILIKE is sufficient)
- Do NOT add real-time features (WebSockets, SSE)

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES (Vitest for frontend, cargo test for backend)
- **Automated tests**: Tests-after (implement features first, then e2e)
- **Framework**: Playwright (e2e), Vitest (unit), cargo test (backend)
- **E2e scope**: Critical paths only (post CRUD, search, comments, theme)

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.omo/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Frontend/UI**: Use Playwright — Navigate, interact, assert DOM, screenshot
- **API/Backend**: Use Bash (curl) — Send requests, assert status + response fields
- **Docker**: Use Bash — Build images, run containers, verify health

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — Stability + Foundation):
├── Task 1: Fix frontend Dockerfile [quick]
├── Task 2: Add .dockerignore files [quick]
├── Task 3: Fix model inconsistencies [quick]
├── Task 4: Clean up dead code [quick]
├── Task 5: Clean up hardcoded secrets [quick]

Wave 2 (After Wave 1 — Backend features + DB):
├── Task 6: Tags DB schema + migration [unspecified-high]
├── Task 7: Comments DB schema + migration [unspecified-high]
├── Task 8: Search API endpoint [unspecified-high]
├── Task 9: Tags API handlers [unspecified-high]
├── Task 10: Comments API handlers [unspecified-high]
├── Task 11: Add markdown rendering dependencies [quick]

Wave 3 (After Wave 2 — Frontend features):
├── Task 12: Tags UI (create, assign, filter) [visual-engineering]
├── Task 13: Search UI (search bar + results) [visual-engineering]
├── Task 14: Comments UI (form + list + admin approval) [visual-engineering]
├── Task 15: Theme toggle (dark/light) [visual-engineering]
├── Task 16: Rich markdown renderer component [visual-engineering]

Wave 4 (After Wave 3 — Quality + Integration):
├── Task 17: Playwright e2e tests [unspecified-high]
├── Task 18: Integration testing + bug fixes [deep]
├── Task 19: Final Docker verification [quick]

Wave FINAL (After ALL tasks):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
├── Task F4: Scope fidelity check (deep)
-> Present results -> Get explicit user okay

Critical Path: T1 → T6/T7 → T9/T10 → T12/T14 → T17 → F1-F4
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 6 (Wave 2)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | — | 2, 3, 4, 5, 6-11, 12-19 |
| 2 | — | 19 |
| 3 | — | 6, 7, 8, 9, 10 |
| 4 | — | 12-16 |
| 5 | — | — |
| 6 | 3 | 9, 12 |
| 7 | 3 | 10, 14 |
| 8 | 3 | 13 |
| 9 | 6 | 12 |
| 10 | 7 | 14 |
| 11 | — | 16 |
| 12 | 9 | 17 |
| 13 | 8 | 17 |
| 14 | 10 | 17 |
| 15 | — | 17 |
| 16 | 11 | 17 |
| 17 | 12-16 | 18 |
| 18 | 17 | 19 |
| 19 | 2, 18 | F1-F4 |

### Agent Dispatch Summary

- **Wave 1**: 5 tasks — T1-T5 → `quick`
- **Wave 2**: 6 tasks — T6-T10 → `unspecified-high`, T11 → `quick`
- **Wave 3**: 5 tasks — T12-T16 → `visual-engineering`
- **Wave 4**: 3 tasks — T17 → `unspecified-high`, T18 → `deep`, T19 → `quick`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [x] 1. Fix Frontend Dockerfile

  **What to do**:
  - Create `frontend/Dockerfile` — multi-stage build (node:20-alpine for build, nginx:alpine to serve dist/)
  - Stage 1: `npm ci` + `npm run build` → produces `dist/`
  - Stage 2: Copy `dist/` to nginx html directory, expose port 80
  - Verify `docker-compose build frontend` succeeds

  **Must NOT do**:
  - Do NOT use `node:latest` — pin to `node:20-alpine`
  - Do NOT run Vite dev server in production image
  - Do NOT include `node_modules` in final image

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
    - No specialized skills needed — standard Dockerfile creation

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4, 5)
  - **Blocks**: Task 19 (Docker verification)
  - **Blocked By**: None

  **References**:
  - `backend/Dockerfile` — Multi-stage build pattern to follow
  - `docker-compose.yml:48-61` — Frontend service config (port 5173, depends on backend)
  - `frontend/package.json` — Build scripts: `npm run build` runs `vue-tsc -b && vite build`
  - `frontend/vite.config.ts` — Build output config

  **Acceptance Criteria**:
  - [ ] `frontend/Dockerfile` exists with multi-stage build
  - [ ] `docker-compose build frontend` succeeds
  - [ ] Final image uses nginx:alpine (not node)

  **QA Scenarios**:
  ```
  Scenario: Docker build succeeds
    Tool: Bash
    Steps:
      1. Run `docker-compose build frontend`
      2. Assert exit code 0
      3. Run `docker images | grep frontend`
      4. Assert image exists
    Expected Result: Image built successfully
    Evidence: .omo/evidence/task-1-docker-build.txt

  Scenario: Container serves static files
    Tool: Bash
    Steps:
      1. Run `docker-compose up -d frontend`
      2. Wait 5s for container start
      3. Run `curl -s http://localhost:5173`
      4. Assert response contains HTML
    Expected Result: Frontend serves HTML from nginx
    Evidence: .omo/evidence/task-1-container-serve.txt
  ```

  **Commit**: YES
  - Message: `fix(infra): add frontend Dockerfile with nginx`
  - Files: `frontend/Dockerfile`
  - Pre-commit: `docker-compose build frontend`

- [x] 2. Add .dockerignore Files

  **What to do**:
  - Create `.dockerignore` in root directory
  - Create `.dockerignore` in `backend/` directory
  - Root: exclude `node_modules/`, `dist/`, `target/`, `.git/`, `.env`, `*.md`
  - Backend: exclude `target/`, `test_backups/`, `backups/`

  **Must NOT do**:
  - Do NOT exclude `Cargo.toml` or `package.json` (needed for build)
  - Do NOT exclude `src/` directories

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4, 5)
  - **Blocks**: Task 19 (Docker verification)
  - **Blocked By**: None

  **References**:
  - `.gitignore` — Pattern reference for what to exclude
  - `backend/Dockerfile` — Check what files are needed for build

  **Acceptance Criteria**:
  - [ ] `.dockerignore` exists at root
  - [ ] `.dockerignore` exists at `backend/`
  - [ ] Both exclude `node_modules/`, `target/`, `.git/`

  **QA Scenarios**:
  ```
  Scenario: Dockerignore reduces build context
    Tool: Bash
    Steps:
      1. Run `docker-compose build --no-cache 2>&1 | grep "Sending build context"`
      2. Assert context size is reasonable (<100MB)
    Expected Result: Build context is small
    Evidence: .omo/evidence/task-2-build-context.txt
  ```

  **Commit**: YES (groups with Task 1)
  - Message: `fix(infra): add .dockerignore files`
  - Files: `.dockerignore`, `backend/.dockerignore`

- [x] 3. Fix Model Inconsistencies

  **What to do**:
  - Read `backend/src/models/` — identify `pull.rs` and `pull_comment.rs`
  - Check if they are duplicates of `pull_request.rs` and `pull_request_comment.rs`
  - If duplicates: delete `pull.rs` and `pull_comment.rs`
  - If different: add to `models/mod.rs` with `pub mod`
  - Verify `cargo build` succeeds

  **Must NOT do**:
  - Do NOT break existing pull request functionality
  - Do NOT rename existing model files without updating all references

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4, 5)
  - **Blocks**: Tasks 6, 7, 8, 9, 10 (backend features)
  - **Blocked By**: None

  **References**:
  - `backend/src/models/mod.rs` — Current model declarations
  - `backend/src/models/pull.rs` — Check if duplicate
  - `backend/src/models/pull_request.rs` — Check if duplicate
  - `backend/src/models/pull_comment.rs` — Check if duplicate
  - `backend/src/models/pull_request_comment.rs` — Check if duplicate

  **Acceptance Criteria**:
  - [ ] No duplicate model files exist
  - [ ] `models/mod.rs` declares all needed models
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Models compile cleanly
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo build 2>&1`
      2. Assert exit code 0
      3. Assert no warnings about unused imports
    Expected Result: Clean compilation
    Evidence: .omo/evidence/task-3-cargo-build.txt
  ```

  **Commit**: YES
  - Message: `fix(models): remove duplicate model files`
  - Files: `backend/src/models/`

- [x] 4. Clean Up Dead Code

  **What to do**:
  - Delete `frontend/src/components/HelloWorld.vue` (unused scaffold leftover)
  - Search for any imports of HelloWorld and remove them
  - Verify `npm run build` succeeds

  **Must NOT do**:
  - Do NOT delete other components that appear unused without verifying

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 5)
  - **Blocks**: Tasks 12-16 (frontend features)
  - **Blocked By**: None

  **References**:
  - `frontend/src/components/HelloWorld.vue` — File to delete
  - Search for `HelloWorld` imports across codebase

  **Acceptance Criteria**:
  - [ ] `HelloWorld.vue` deleted
  - [ ] No imports reference HelloWorld
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Build succeeds after cleanup
    Tool: Bash
    Steps:
      1. Run `cd frontend && npm run build 2>&1`
      2. Assert exit code 0
      3. Assert no errors in output
    Expected Result: Clean build
    Evidence: .omo/evidence/task-4-build.txt
  ```

  **Commit**: YES
  - Message: `chore: remove unused HelloWorld component`
  - Files: `frontend/src/components/HelloWorld.vue`

- [x] 5. Clean Up Hardcoded Secrets

  **What to do**:
  - Read `run.txt` — identify hardcoded admin key
  - Replace with placeholder: `ADMIN_KEY=your_admin_key_here`
  - Add comment explaining where to set the key
  - Verify `.env` is in `.gitignore`

  **Must NOT do**:
  - Do NOT change `.env.example` (it already has proper placeholder)
  - Do NOT remove the startup commands from `run.txt`

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 4)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `run.txt` — Contains hardcoded key to replace
  - `.gitignore` — Verify `.env` is excluded

  **Acceptance Criteria**:
  - [ ] `run.txt` contains placeholder, not real key
  - [ ] `.env` is in `.gitignore`

  **QA Scenarios**:
  ```
  Scenario: No hardcoded secrets
    Tool: Bash (grep)
    Steps:
      1. Run `grep -r "zyblog_admin_2024" . --include="*.txt" --include="*.md"`
      2. Assert no matches found
    Expected Result: No hardcoded secrets in tracked files
    Evidence: .omo/evidence/task-5-no-secrets.txt
  ```

  **Commit**: YES
  - Message: `security: remove hardcoded admin key from run.txt`
  - Files: `run.txt`

- [x] 6. Tags DB Schema + Migration

  **What to do**:
  - Create SeaORM migration for `tags` table: `id SERIAL PRIMARY KEY, name VARCHAR NOT NULL UNIQUE, slug VARCHAR NOT NULL UNIQUE, created_at TIMESTAMPTZ`
  - Create SeaORM migration for `post_tags` join table: `post_id INT REFERENCES posts(id) ON DELETE CASCADE, tag_id INT REFERENCES tags(id) ON DELETE CASCADE, PRIMARY KEY (post_id, tag_id)`
  - Create SeaORM entity `models/tag.rs` with `#[derive(DeriveEntityModel)]`
  - Create SeaORM entity `models/post_tag.rs` for the join table
  - Add `pub mod tag; pub mod post_tag;` to `models/mod.rs`
  - Run migration and verify tables exist

  **Must NOT do**:
  - Do NOT modify existing `posts` table schema
  - Do NOT add tag columns to posts table (use join table)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 7, 8, 9, 10, 11)
  - **Blocks**: Task 9 (Tags API), Task 12 (Tags UI)
  - **Blocked By**: Task 3 (model cleanup)

  **References**:
  - `backend/src/models/post.rs` — Entity definition pattern to follow
  - `backend/src/migrations/` — Migration file pattern
  - SeaORM docs: Entity definition with relations

  **Acceptance Criteria**:
  - [ ] `tags` table exists in database
  - [ ] `post_tags` join table exists with foreign keys
  - [ ] `models/tag.rs` and `models/post_tag.rs` exist
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Migration runs successfully
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo run -- migrate`
      2. Assert exit code 0
      3. Run `psql -c "\dt"` to list tables
      4. Assert `tags` and `post_tags` tables exist
    Expected Result: Tables created
    Evidence: .omo/evidence/task-6-migration.txt
  ```

  **Commit**: YES
  - Message: `feat(db): add tags and post_tags schema`
  - Files: `backend/src/migrations/`, `backend/src/models/tag.rs`, `backend/src/models/post_tag.rs`

- [x] 7. Comments DB Schema + Migration

  **What to do**:
  - Create SeaORM migration for `comments` table:
    - `id SERIAL PRIMARY KEY`
    - `post_id INT REFERENCES posts(id) ON DELETE CASCADE`
    - `author_name VARCHAR NOT NULL`
    - `author_email VARCHAR`
    - `content TEXT NOT NULL`
    - `approved BOOLEAN DEFAULT FALSE`
    - `created_at TIMESTAMPTZ`
    - `updated_at TIMESTAMPTZ`
  - Create SeaORM entity `models/comment.rs`
  - Add `pub mod comment;` to `models/mod.rs`
  - Run migration and verify table exists

  **Must NOT do**:
  - Do NOT add user_id foreign key (no user accounts)
  - Do NOT auto-approve comments (moderation required)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 8, 9, 10, 11)
  - **Blocks**: Task 10 (Comments API), Task 14 (Comments UI)
  - **Blocked By**: Task 3 (model cleanup)

  **References**:
  - `backend/src/models/post.rs` — Entity pattern
  - `backend/src/migrations/` — Migration pattern

  **Acceptance Criteria**:
  - [ ] `comments` table exists with all columns
  - [ ] `models/comment.rs` exists
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Comments table created
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo run -- migrate`
      2. Run `psql -c "\d comments"`
      3. Assert columns: id, post_id, author_name, content, approved, created_at
    Expected Result: Table schema matches spec
    Evidence: .omo/evidence/task-7-comments-table.txt
  ```

  **Commit**: YES
  - Message: `feat(db): add comments schema with moderation`
  - Files: `backend/src/migrations/`, `backend/src/models/comment.rs`

- [x] 8. Search API Endpoint

  **What to do**:
  - Add `GET /api/v1/posts/search?q={query}` handler in `handlers/posts.rs`
  - Query: `SELECT * FROM posts WHERE title ILIKE '%query%' OR content ILIKE '%query%'`
  - Return same `PostListResponse` format as `list_posts`
  - Add utoipa annotation for OpenAPI docs
  - Register route in `main.rs`

  **Must NOT do**:
  - Do NOT implement full-text search (ILIKE is sufficient)
  - Do NOT search across tags or comments (posts only)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7, 9, 10, 11)
  - **Blocks**: Task 13 (Search UI)
  - **Blocked By**: Task 3 (model cleanup)

  **References**:
  - `backend/src/handlers/posts.rs:list_posts` — Pagination and response pattern
  - `backend/src/models/post.rs` — Post entity for query building

  **Acceptance Criteria**:
  - [ ] `GET /api/v1/posts/search?q=term` returns matching posts
  - [ ] Response format matches `PostListResponse`
  - [ ] OpenAPI docs include search endpoint

  **QA Scenarios**:
  ```
  Scenario: Search returns matching posts
    Tool: Bash (curl)
    Steps:
      1. Create a post with title "Test Search Post"
      2. Run `curl http://localhost:8080/api/v1/posts/search?q=Test+Search`
      3. Assert status 200
      4. Assert response contains "Test Search Post"
    Expected Result: Search finds matching post
    Evidence: .omo/evidence/task-8-search.txt

  Scenario: Search returns empty for no match
    Tool: Bash (curl)
    Steps:
      1. Run `curl http://localhost:8080/api/v1/posts/search?q=nonexistent_xyz`
      2. Assert status 200
      3. Assert items array is empty
    Expected Result: Empty results for no match
    Evidence: .omo/evidence/task-8-search-empty.txt
  ```

  **Commit**: YES
  - Message: `feat(api): add post search endpoint`
  - Files: `backend/src/handlers/posts.rs`, `backend/src/main.rs`

- [x] 9. Tags API Handlers

  **What to do**:
  - Create `handlers/tags.rs` with endpoints:
    - `POST /api/v1/tags` — Create tag (auth required)
    - `GET /api/v1/tags` — List all tags (public)
    - `DELETE /api/v1/tags/:id` — Delete tag (auth required)
  - Create `routes/tags.rs` for route nesting
  - Register in `main.rs`
  - Add utoipa annotations

  **Must NOT do**:
  - Do NOT add tag editing (name is immutable once created)
  - Do NOT add bulk operations

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7, 8, 10, 11)
  - **Blocks**: Task 12 (Tags UI)
  - **Blocked By**: Task 6 (tags schema)

  **References**:
  - `backend/src/handlers/posts.rs` — Handler pattern
  - `backend/src/routes/subscribers.rs` — Route nesting pattern
  - `backend/src/main.rs` — Route registration

  **Acceptance Criteria**:
  - [ ] `POST /api/v1/tags` creates a tag
  - [ ] `GET /api/v1/tags` lists all tags
  - [ ] `DELETE /api/v1/tags/:id` removes a tag
  - [ ] OpenAPI docs include all tag endpoints

  **QA Scenarios**:
  ```
  Scenario: CRUD operations work
    Tool: Bash (curl)
    Steps:
      1. Create: `curl -X POST -H "Authorization: Bearer $KEY" -d '{"name":"Rust"}' http://localhost:8080/api/v1/tags`
      2. Assert status 201, response has id and name
      3. List: `curl http://localhost:8080/api/v1/tags`
      4. Assert status 200, array contains "Rust"
      5. Delete: `curl -X DELETE -H "Authorization: Bearer $KEY" http://localhost:8080/api/v1/tags/{id}`
      6. Assert status 204
    Expected Result: Full CRUD cycle works
    Evidence: .omo/evidence/task-9-tags-crud.txt
  ```

  **Commit**: YES
  - Message: `feat(api): add tags CRUD endpoints`
  - Files: `backend/src/handlers/tags.rs`, `backend/src/routes/tags.rs`, `backend/src/main.rs`

- [x] 10. Comments API Handlers

  **What to do**:
  - Create `handlers/comments.rs` with endpoints:
    - `POST /api/v1/posts/:id/comments` — Submit comment (public, no auth)
    - `GET /api/v1/posts/:id/comments` — List approved comments (public)
    - `GET /api/v1/comments/pending` — List pending comments (auth required)
    - `PUT /api/v1/comments/:id/approve` — Approve comment (auth required)
    - `DELETE /api/v1/comments/:id` — Delete comment (auth required)
  - Create `routes/comments.rs`
  - Register in `main.rs`
  - Add utoipa annotations

  **Must NOT do**:
  - Do NOT auto-approve comments
  - Do NOT expose unapproved comments to public GET endpoint

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7, 8, 9, 11)
  - **Blocks**: Task 14 (Comments UI)
  - **Blocked By**: Task 7 (comments schema)

  **References**:
  - `backend/src/handlers/posts.rs` — Handler pattern
  - `backend/src/handlers/pulls.rs` — Similar nested resource pattern

  **Acceptance Criteria**:
  - [ ] Public can submit comments
  - [ ] Public GET only returns approved comments
  - [ ] Admin can list pending and approve/delete
  - [ ] OpenAPI docs complete

  **QA Scenarios**:
  ```
  Scenario: Comment moderation flow
    Tool: Bash (curl)
    Steps:
      1. Submit: `curl -X POST -d '{"author_name":"Reader","content":"Great post!"}' http://localhost:8080/api/v1/posts/1/comments`
      2. Assert status 201
      3. Public list: `curl http://localhost:8080/api/v1/posts/1/comments`
      4. Assert empty (not yet approved)
      5. Pending list: `curl -H "Authorization: Bearer $KEY" http://localhost:8080/api/v1/comments/pending`
      6. Assert contains the comment
      7. Approve: `curl -X PUT -H "Authorization: Bearer $KEY" http://localhost:8080/api/v1/comments/{id}/approve`
      8. Public list: `curl http://localhost:8080/api/v1/posts/1/comments`
      9. Assert contains approved comment
    Expected Result: Full moderation flow works
    Evidence: .omo/evidence/task-10-comment-flow.txt
  ```

  **Commit**: YES
  - Message: `feat(api): add comments with moderation flow`
  - Files: `backend/src/handlers/comments.rs`, `backend/src/routes/comments.rs`, `backend/src/main.rs`

- [x] 11. Add Markdown Rendering Dependencies

  **What to do**:
  - Install frontend packages: `npm install markdown-it highlight.js katex mermaid`
  - Install types: `npm install -D @types/markdown-it`
  - Verify `npm run build` succeeds

  **Must NOT do**:
  - Do NOT install heavy libraries when lightweight alternatives exist

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7, 8, 9, 10)
  - **Blocks**: Task 16 (Markdown renderer)
  - **Blocked By**: None

  **References**:
  - `frontend/package.json` — Current dependencies

  **Acceptance Criteria**:
  - [ ] `markdown-it` in package.json
  - [ ] `highlight.js` in package.json
  - [ ] `katex` in package.json
  - [ ] `mermaid` in package.json
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Dependencies installed
    Tool: Bash
    Steps:
      1. Run `cd frontend && npm ls markdown-it highlight.js katex mermaid`
      2. Assert all 4 packages listed
    Expected Result: All markdown dependencies present
    Evidence: .omo/evidence/task-11-deps.txt
  ```

  **Commit**: YES
  - Message: `feat(deps): add markdown rendering libraries`
  - Files: `frontend/package.json`

- [x] 12. Tags UI (Create, Assign, Filter)

  **What to do**:
  - Create `composables/useTags.ts` — API calls for tags CRUD
  - Add tag selector to `Publish.vue` — multi-select dropdown for assigning tags to posts
  - Add tag display to `PostDetail.vue` — show tags as badges
  - Add tag filter to `Home.vue` — click tag to filter posts
  - Style with Naive UI `n-tag` components

  **Must NOT do**:
  - Do NOT add tag management page (inline in publish is sufficient)
  - Do NOT add tag auto-complete (simple dropdown)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 13, 14, 15, 16)
  - **Blocks**: Task 17 (e2e tests)
  - **Blocked By**: Task 9 (Tags API)

  **References**:
  - `frontend/src/composables/usePosts.ts` — Composable pattern
  - `frontend/src/views/Publish.vue` — Post creation form
  - `frontend/src/views/Home.vue` — Post list
  - `frontend/src/views/PostDetail.vue` — Post detail page
  - `frontend/src/lib/api.ts` — API client

  **Acceptance Criteria**:
  - [ ] Tags can be created in publish form
  - [ ] Tags display as badges on post detail
  - [ ] Clicking tag in list filters posts by that tag
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Tags workflow
    Tool: Playwright
    Steps:
      1. Navigate to /publish
      2. Create a post with tag "Rust"
      3. Navigate to post detail
      4. Assert "Rust" tag badge visible
      5. Navigate to home
      6. Click "Rust" tag
      7. Assert post list filters to show only Rust posts
    Expected Result: Tags flow works end-to-end
    Evidence: .omo/evidence/task-12-tags-ui.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add tags create, assign, and filter`
  - Files: `frontend/src/composables/useTags.ts`, `frontend/src/views/Publish.vue`, `frontend/src/views/Home.vue`, `frontend/src/views/PostDetail.vue`

- [x] 13. Search UI (Search Bar + Results)

  **What to do**:
  - Add search bar to `Layout.vue` or `Home.vue` header area
  - Create `composables/useSearch.ts` — API call to search endpoint
  - Show search results in the post list area (reuse existing post card)
  - Add URL query param support (`?q=searchterm`) for shareable search links
  - Debounce search input (300ms)

  **Must NOT do**:
  - Do NOT add search suggestions/autocomplete
  - Do NOT add search filters (just text search)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 12, 14, 15, 16)
  - **Blocks**: Task 17 (e2e tests)
  - **Blocked By**: Task 8 (Search API)

  **References**:
  - `frontend/src/views/Home.vue` — Post list to integrate search
  - `frontend/src/composables/usePosts.ts` — Composable pattern
  - Naive UI `n-input` component for search bar

  **Acceptance Criteria**:
  - [ ] Search bar visible in header/home
  - [ ] Typing triggers debounced search
  - [ ] Results display in post list format
  - [ ] URL updates with `?q=` param

  **QA Scenarios**:
  ```
  Scenario: Search works
    Tool: Playwright
    Steps:
      1. Navigate to /
      2. Type "test" in search bar
      3. Wait 500ms for debounce
      4. Assert results list updates
      5. Assert URL contains `?q=test`
    Expected Result: Search returns filtered results
    Evidence: .omo/evidence/task-13-search-ui.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add search bar with debounced results`
  - Files: `frontend/src/composables/useSearch.ts`, `frontend/src/views/Home.vue`, `frontend/src/components/Layout.vue`

- [x] 14. Comments UI (Form + List + Admin Approval)

  **What to do**:
  - Create `composables/useComments.ts` — API calls for comments
  - Add comment form to `PostDetail.vue` — name, email (optional), content fields
  - Add comment list below post content — show approved comments only
  - Add admin comment management view or section — list pending, approve/delete buttons
  - Use Naive UI `n-form`, `n-comment`, `n-button` components

  **Must NOT do**:
  - Do NOT show unapproved comments to public
  - Do NOT add rich text editing for comments (plain text only)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 12, 13, 15, 16)
  - **Blocks**: Task 17 (e2e tests)
  - **Blocked By**: Task 10 (Comments API)

  **References**:
  - `frontend/src/views/PostDetail.vue` — Post detail page to add comments
  - `frontend/src/composables/usePullRequests.ts` — Similar interaction pattern
  - Naive UI comment components

  **Acceptance Criteria**:
  - [ ] Comment form visible on post detail page
  - [ ] Submitted comments appear after approval
  - [ ] Admin can see pending comments and approve/delete
  - [ ] Public users cannot see unapproved comments

  **QA Scenarios**:
  ```
  Scenario: Comment submission and approval
    Tool: Playwright
    Steps:
      1. Navigate to post detail page
      2. Fill comment form: name="Test User", content="Great post!"
      3. Click submit
      4. Assert success message
      5. Assert comment NOT visible (pending approval)
      6. Login as admin
      7. Navigate to comment management
      8. Approve the comment
      9. Navigate back to post detail
      10. Assert comment visible
    Expected Result: Full comment flow works
    Evidence: .omo/evidence/task-14-comments-ui.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add comments with moderation`
  - Files: `frontend/src/composables/useComments.ts`, `frontend/src/views/PostDetail.vue`

- [x] 15. Theme Toggle (Dark/Light Mode)

  **What to do**:
  - Add theme toggle button in `Layout.vue` header
  - Use Naive UI's built-in dark theme (`darkTheme` from naive-ui)
  - Persist theme preference in `localStorage('zyblog_theme')`
  - Apply theme via `n-config-provider` wrapping the app
  - Respect system preference on first visit (`prefers-color-scheme`)

  **Must NOT do**:
  - Do NOT create custom CSS theme (use Naive UI's built-in)
  - Do NOT add theme customization (just dark/light toggle)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 12, 13, 14, 16)
  - **Blocks**: Task 17 (e2e tests)
  - **Blocked By**: None

  **References**:
  - `frontend/src/App.vue` — Root component to wrap with n-config-provider
  - `frontend/src/components/Layout.vue` — Header to add toggle button
  - Naive UI docs: Dark theme usage

  **Acceptance Criteria**:
  - [ ] Theme toggle button visible in header
  - [ ] Clicking toggle switches between dark and light
  - [ ] Theme persists across page reload
  - [ ] System preference respected on first visit

  **QA Scenarios**:
  ```
  Scenario: Theme toggle works
    Tool: Playwright
    Steps:
      1. Navigate to /
      2. Click theme toggle button
      3. Assert page background changes color
      4. Reload page
      5. Assert theme persisted (still dark/light)
    Expected Result: Theme toggles and persists
    Evidence: .omo/evidence/task-15-theme-toggle.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add dark/light theme toggle`
  - Files: `frontend/src/App.vue`, `frontend/src/components/Layout.vue`

- [x] 16. Rich Markdown Renderer Component

  **What to do**:
  - Create `components/MarkdownRenderer.vue`
  - Use markdown-it with plugins:
    - `markdown-it-highlightjs` for code syntax highlighting
    - `markdown-it-katex` for LaTeX math formulas
    - `markdown-it-mermaid` for mermaid diagrams
  - Apply to `PostDetail.vue` — render post content as rich markdown
  - Style code blocks with highlight.js theme (e.g., github-dark)
  - Ensure images in markdown are responsive

  **Must NOT do**:
  - Do NOT implement custom markdown parser (use markdown-it)
  - Do NOT add WYSIWYG editing (render only)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 12, 13, 14, 15)
  - **Blocks**: Task 17 (e2e tests)
  - **Blocked By**: Task 11 (markdown deps)

  **References**:
  - `frontend/src/views/PostDetail.vue` — Where to render markdown
  - `frontend/src/components/VideoPlayer.vue` — Component pattern
  - markdown-it plugin docs

  **Acceptance Criteria**:
  - [ ] Markdown renders with syntax highlighting
  - [ ] LaTeX formulas render correctly
  - [ ] Mermaid diagrams render
  - [ ] Images are responsive

  **QA Scenarios**:
  ```
  Scenario: Rich markdown renders
    Tool: Playwright
    Steps:
      1. Create a post with markdown content:
         - Code block with syntax
         - LaTeX formula: $E = mc^2$
         - Mermaid diagram
      2. Navigate to post detail
      3. Assert code block has syntax highlighting
      4. Assert LaTeX renders as formatted math
      5. Assert mermaid diagram renders as SVG
    Expected Result: All markdown features render correctly
    Evidence: .omo/evidence/task-16-markdown-render.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add rich markdown renderer with math and diagrams`
  - Files: `frontend/src/components/MarkdownRenderer.vue`, `frontend/src/views/PostDetail.vue`

- [x] 17. Playwright E2E Tests

  **What to do**:
  - Install Playwright: `npm init playwright@latest` in frontend/
  - Create e2e test files:
    - `e2e/posts.spec.ts` — Post CRUD flow
    - `e2e/search.spec.ts` — Search functionality
    - `e2e/tags.spec.ts` — Tags create, assign, filter
    - `e2e/comments.spec.ts` — Comment submission and approval
    - `e2e/theme.spec.ts` — Theme toggle persistence
  - Configure `playwright.config.ts` — base URL, browser settings
  - Add `npm run test:e2e` script to package.json

  **Must NOT do**:
  - Do NOT test every edge case (critical paths only)
  - Do NOT add visual regression testing

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (with Tasks 18, 19)
  - **Blocks**: Task 18 (integration testing)
  - **Blocked By**: Tasks 12-16 (all frontend features)

  **References**:
  - `frontend/src/__tests__/` — Existing test patterns
  - Playwright docs: Test configuration

  **Acceptance Criteria**:
  - [ ] Playwright installed and configured
  - [ ] 5 test files created
  - [ ] `npx playwright test` passes
  - [ ] Test coverage for critical paths

  **QA Scenarios**:
  ```
  Scenario: E2e tests pass
    Tool: Bash
    Steps:
      1. Run `cd frontend && npx playwright test`
      2. Assert exit code 0
      3. Assert all tests pass
    Expected Result: All e2e tests green
    Evidence: .omo/evidence/task-17-e2e-results.txt
  ```

  **Commit**: YES
  - Message: `test(e2e): add Playwright tests for critical paths`
  - Files: `frontend/e2e/`, `frontend/playwright.config.ts`, `frontend/package.json`

- [x] 18. Integration Testing + Bug Fixes

  **What to do**:
  - Run full test suite: `cargo test` + `npm run test` + `npx playwright test`
  - Fix any bugs found during testing
  - Verify cross-feature integration:
    - Tags work with search
    - Comments work with posts
    - Theme applies to all new components
    - Markdown renders in post detail
  - Test edge cases: empty states, long content, special characters

  **Must NOT do**:
  - Do NOT add new features (fix only)
  - Do NOT refactor working code

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (after Task 17)
  - **Blocks**: Task 19 (Docker verification)
  - **Blocked By**: Task 17 (e2e tests)

  **References**:
  - All task QA scenarios — Execute each one

  **Acceptance Criteria**:
  - [ ] `cargo test` passes
  - [ ] `npm run test` passes
  - [ ] `npx playwright test` passes
  - [ ] No known bugs

  **QA Scenarios**:
  ```
  Scenario: Full test suite passes
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo test`
      2. Assert exit code 0
      3. Run `cd frontend && npm run test`
      4. Assert exit code 0
      5. Run `cd frontend && npx playwright test`
      6. Assert exit code 0
    Expected Result: All tests pass
    Evidence: .omo/evidence/task-18-full-suite.txt
  ```

  **Commit**: YES (if fixes needed)
  - Message: `fix(integration): resolve bugs from testing`
  - Files: various (only if fixes needed)

- [x] 19. Final Docker Verification

  **What to do**:
  - Run `docker-compose build --no-cache` — verify both images build
  - Run `docker-compose up -d` — verify all 3 services start
  - Verify health: `curl http://localhost:8080/api/health`
  - Verify frontend: `curl http://localhost:5173`
  - Run migrations inside container: `make migrate`
  - Test API through Docker: create post, search, comment

  **Must NOT do**:
  - Do NOT modify Docker files (just verify)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (after Task 18)
  - **Blocks**: F1-F4 (final verification)
  - **Blocked By**: Tasks 2, 18

  **References**:
  - `docker-compose.yml` — Service configuration
  - `Makefile` — Docker convenience commands

  **Acceptance Criteria**:
  - [ ] `docker-compose build` succeeds for both services
  - [ ] All 3 services start and are healthy
  - [ ] API accessible through Docker
  - [ ] Frontend accessible through Docker

  **QA Scenarios**:
  ```
  Scenario: Docker deployment works
    Tool: Bash
    Steps:
      1. Run `docker-compose build --no-cache`
      2. Assert exit code 0
      3. Run `docker-compose up -d`
      4. Wait 10s for services to start
      5. Run `curl http://localhost:8080/api/health`
      6. Assert response contains "ok"
      7. Run `curl http://localhost:5173`
      8. Assert response contains HTML
    Expected Result: Full Docker stack works
    Evidence: .omo/evidence/task-19-docker-final.txt
  ```

  **Commit**: NO (verification only)

---

## Final Verification Wave

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists. For each "Must NOT Have": search codebase for forbidden patterns. Check evidence files exist. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo test` + `npm run test`. Review all changed files for: `unwrap()` in handlers, missing utoipa annotations, `any` types in TypeScript, unused imports. Check AI slop patterns.
  Output: `Backend Tests [PASS/FAIL] | Frontend Tests [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high` (+ `playwright` skill)
  Start from clean Docker state. Execute EVERY QA scenario from EVERY task. Test cross-task integration (tags + posts, search + tags, comments + posts). Test edge cases.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff. Verify 1:1 — everything in spec was built, nothing beyond spec. Check "Must NOT do" compliance. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

| Wave | Commit | Files |
|------|--------|-------|
| 1 | `fix(infra): add frontend Dockerfile and .dockerignore` | Docker files |
| 1 | `fix(models): clean up model declarations and dead code` | models/, components/ |
| 2 | `feat(db): add tags and comments schema migrations` | migrations/ |
| 2 | `feat(api): add tags, comments, and search endpoints` | handlers/, routes/ |
| 3 | `feat(ui): add tags, search, comments, theme toggle` | views/, components/, composables/ |
| 3 | `feat(ui): add rich markdown renderer` | components/ |
| 4 | `test(e2e): add Playwright tests for critical paths` | e2e/ |
| 4 | `fix(integration): bug fixes from e2e testing` | various |

---

## Success Criteria

### Verification Commands
```bash
# Docker
docker-compose build          # Expected: both services build successfully
docker-compose up -d          # Expected: all 3 services running
curl http://localhost:8080/api/health  # Expected: {"status":"ok"}

# Backend
cd backend && cargo test      # Expected: all tests pass

# Frontend
cd frontend && npm run test   # Expected: all tests pass
cd frontend && npm run build  # Expected: build succeeds

# E2e
cd frontend && npx playwright test  # Expected: all e2e tests pass
```

### Final Checklist
- [ ] `docker-compose build` succeeds
- [ ] Tags CRUD works
- [ ] Search returns results
- [ ] Comments flow (submit → approve → display) works
- [ ] Theme toggle persists
- [ ] Markdown renders with highlighting, math, diagrams
- [ ] All tests pass (unit + e2e)
- [ ] No scope creep detected
