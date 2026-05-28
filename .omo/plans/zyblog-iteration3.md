# ZYBlog Iteration 3 — 用户系统 + 内容管理重构

## TL;DR

> **Quick Summary**: 将 ZYBlog 从单管理员博客转型为多用户协作平台，包含用户认证、版本控制、PR 重构、主页 redesign、TODO 订阅等 7 大功能。
>
> **Deliverables**:
> - JWT + GitHub OAuth 用户认证系统（访客/贡献者/博主三种角色）
> - 文章版本控制（保存、回滚、diff 视图）
> - PR 系统重构（片段选中 + 内联评论 + 直接替换 + 应用）
> - 媒体文件管理（元数据表 + StorageAdapter trait）
> - 主页 redesign（Hero 动画 + 文章卡片网格 + 分类筛选）
> - TODO 管理 + 订阅（博主管理、读者订阅、邮件通知）
>
> **Estimated Effort**: XL
> **Parallel Execution**: YES — 4 waves
> **Critical Path**: Users → Auth → Version Control → PR Redesign → Homepage

---

## Context

### Original Request
用户请求迭代 3，包含 6 大功能：用户账号系统、博客编辑、PR 重构、存储优化、主页 redesign、TODO/订阅。

### Interview Summary
**Key Discussions**:
- **认证**: 邮箱+密码 & GitHub OAuth，JWT (access+refresh token)
- **用户组**: 访客(visitor)、贡献者(contributor)、博主(admin)
- **PR**: 渲染后选中文字 → 自动映射到源文本位置，片段替换 + 说明
- **版本控制**: 完整版本控制（保存+回滚+diff+分支）
- **存储**: 保持 PostgreSQL TEXT 存储，新增 media 表 + StorageAdapter trait
- **主页**: CSS+JS Hero 动画 + 文章卡片网格 + 分类筛选（参考 mimo.xiaomi.com）
- **TODO**: 博主管理所有 TODO，读者订阅，完成后邮件通知
- **测试**: TDD（RED-GREEN-REFACTOR）

### Metis Review
**Identified Gaps** (addressed):
- 认证迁移策略：实现双认证中间件（ADMIN_KEY + JWT）过渡
- JWT Secret 管理：新增 JWT_SECRET、GITHUB_CLIENT_ID/SECRET 等环境变量
- GitHub OAuth 账号关联：邮箱作为唯一标识，相同邮箱自动关联
- 版本控制范围：线性版本历史，不实现真正的 Git 分支
- PR 片段映射：基于行号+列号的位置映射，使用 `similar` crate 做 diff
- 速率限制：本迭代不实现，defer 到下一迭代
- 密码重置：本迭代不实现，defer 到下一迭代

---

## Work Objectives

### Core Objective
将 ZYBlog 从单管理员博客转型为多用户协作平台，支持用户注册登录、文章版本控制、协作编辑、TODO 订阅等现代博客功能。

### Concrete Deliverables
- `POST /api/v1/auth/register` — 用户注册
- `POST /api/v1/auth/login` — 用户登录（返回 JWT）
- `GET /api/v1/auth/github` — GitHub OAuth 跳转
- `GET /api/v1/auth/github/callback` — GitHub OAuth 回调
- `POST /api/v1/posts/:id/revisions` — 创建版本快照
- `GET /api/v1/posts/:id/revisions/:rev_id/rollback` — 回滚到指定版本
- `GET /api/v1/posts/:id/diff?from=&to=` — 版本 diff
- `POST /api/v1/posts/:id/pulls` — 创建 PR（片段替换）
- `POST /api/v1/pulls/:id/apply` — 应用 PR
- `POST /api/v1/media` — 上传媒体文件
- `GET /api/v1/todos` — 获取 TODO 列表
- `POST /api/v1/todos/:id/subscribe` — 订阅 TODO
- 全新主页（Hero 动画 + 卡片网格 + 分类筛选）

### Definition of Done
- [ ] `cargo test` 全部通过
- [ ] `npm run test` 全部通过
- [ ] `cargo build` 无错误
- [ ] `npm run build` 无错误
- [ ] 所有新 API 端点有 utoipa 注解
- [ ] 所有新端点有 TDD 测试

### Must Have
- JWT + GitHub OAuth 用户认证
- 三种用户角色（visitor/contributor/admin）
- 文章版本控制（保存+回滚+diff）
- PR 片段选中 + 内联评论 + 应用
- 媒体文件元数据管理
- 主页 Hero 动画 + 卡片网格 + 分类筛选
- TODO 管理 + 订阅 + 邮件通知
- 博客编辑（已发布+草稿）

### Must NOT Have (Guardrails)
- Do NOT 添加用户间消息系统
- Do NOT 添加实时协作编辑（WebSocket）
- Do NOT 重新设计 admin dashboard（只添加新功能）
- Do NOT 添加移动端 app
- Do NOT 添加 i18n
- Do NOT 添加访问统计/分析
- Do NOT 重构富文本编辑器
- Do NOT 添加通知偏好设置（仅邮件）
- Do NOT 实现密码重置（defer）
- Do NOT 添加速率限制（defer）
- Do NOT 实现真正的 Git 分支（线性版本历史足够）
- Do NOT 添加图片处理/视频转码/CDN 集成
- Do NOT 引入 UUID（保持 i32 ID）
- Do NOT 引入新的 UI 库（保持 Naive UI）
- Do NOT 使用 Options API（保持 Composition API）

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES (Vitest for frontend, cargo test for backend)
- **Automated tests**: TDD (RED-GREEN-REFACTOR)
- **Framework**: cargo test (backend), Vitest (frontend), Playwright (e2e)
- **TDD scope**: 每个功能先写测试再实现

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.omo/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Backend API**: Use Bash (curl) — Send requests, assert status + response fields
- **Frontend UI**: Use Playwright — Navigate, interact, assert DOM
- **Auth flows**: Use Bash (curl) — Test JWT generation, validation, role access

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Foundation — Users + Auth):
├── T1: Users DB schema + migration [unspecified-high]
├── T2: JWT auth middleware [unspecified-high]
├── T3: Auth handlers (register/login/refresh) [unspecified-high]
├── T4: GitHub OAuth handlers [unspecified-high]
├── T5: Frontend auth composables + login page [visual-engineering]
├── T6: Frontend route guards [visual-engineering]

Wave 2 (Core Features — parallel):
├── T7: Version control DB schema [unspecified-high]
├── T8: Version control handlers [unspecified-high]
├── T9: Media DB schema + StorageAdapter trait [unspecified-high]
├── T10: Media upload handlers [unspecified-high]
├── T11: PR redesign DB schema [unspecified-high]
├── T12: PR fragment mapping algorithm [deep]
├── T13: PR handlers (create/apply/inline comments) [unspecified-high]
├── T14: Frontend version control UI [visual-engineering]
├── T15: Frontend media manager UI [visual-engineering]
├── T16: Frontend PR redesign UI [visual-engineering]

Wave 3 (Integration — TODO + Homepage):
├── T17: TODO subscription DB schema [unspecified-high]
├── T18: TODO handlers (list/subscribe/complete) [unspecified-high]
├── T19: TODO email notification [unspecified-high]
├── T20: Homepage Hero animation [visual-engineering]
├── T21: Homepage article card grid [visual-engineering]
├── T22: Homepage category filter [visual-engineering]
├── T23: Frontend TODO management UI [visual-engineering]
├── T24: Frontend TODO subscription UI [visual-engineering]

Wave 4 (Polish + Final):
├── T25: Auth migration (ADMIN_KEY → JWT transition) [unspecified-high]
├── T26: Blog editing integration (edit + auto-save revision) [unspecified-high]
├── T27: E2e tests (Playwright) [unspecified-high]
├── T28: Integration testing + bug fixes [deep]

Wave FINAL:
├── F1: Plan compliance audit (oracle)
├── F2: Code quality review (unspecified-high)
├── F3: Real manual QA (unspecified-high)
├── F4: Scope fidelity check (deep)

Critical Path: T1 → T2 → T3 → T7 → T8 → T12 → T13 → T20 → F1-F4
Parallel Speedup: ~65% faster than sequential
Max Concurrent: 10 (Wave 2)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| T1 | — | T2, T3, T4, T7, T9, T11, T17 |
| T2 | T1 | T3, T4, T5, T6 |
| T3 | T2 | T5, T25 |
| T4 | T2 | T5 |
| T5 | T3, T4 | T6, T20-T24 |
| T6 | T5 | — |
| T7 | T1 | T8, T14 |
| T8 | T7 | T14, T26 |
| T9 | T1 | T10, T15 |
| T10 | T9 | T15 |
| T11 | T1 | T12, T13, T16 |
| T12 | T11 | T13, T16 |
| T13 | T12 | T16 |
| T14 | T8 | T27 |
| T15 | T10 | T27 |
| T16 | T13 | T27 |
| T17 | T1 | T18, T19, T23, T24 |
| T18 | T17 | T19, T23 |
| T19 | T18 | T24 |
| T20 | T5 | T27 |
| T21 | T5 | T27 |
| T22 | T5 | T27 |
| T23 | T18 | T27 |
| T24 | T19 | T27 |
| T25 | T3 | T26 |
| T26 | T8, T25 | T27 |
| T27 | T14-T16, T20-T24 | T28 |
| T28 | T27 | F1-F4 |

### Agent Dispatch Summary

- **Wave 1**: 6 tasks — T1-T4 → `unspecified-high`, T5-T6 → `visual-engineering`
- **Wave 2**: 10 tasks — T7-T13 → `unspecified-high`/`deep`, T14-T16 → `visual-engineering`
- **Wave 3**: 8 tasks — T17-T19 → `unspecified-high`, T20-T24 → `visual-engineering`
- **Wave 4**: 4 tasks — T25-T27 → `unspecified-high`, T28 → `deep`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

### Wave 1: Foundation (Users + Auth)

- [x] 1. Users DB Schema + Migration

  **What to do**:
  - Create migration `m20240101_000011_create_users_table.rs`
  - Schema: `id SERIAL PRIMARY KEY, email VARCHAR(255) UNIQUE NOT NULL, password_hash VARCHAR(255), name VARCHAR(255) NOT NULL, avatar_url VARCHAR(500), role VARCHAR(20) NOT NULL DEFAULT 'visitor', github_id VARCHAR(100) UNIQUE, created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
  - Create SeaORM entity `models/user.rs`
  - Add `pub mod user;` to `models/mod.rs`
  - Create migration `m20240101_000012_create_refresh_tokens_table.rs`
  - Schema: `id SERIAL PRIMARY KEY, user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE, token_hash VARCHAR(255) NOT NULL, expires_at TIMESTAMPTZ NOT NULL, created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
  - Create SeaORM entity `models/refresh_token.rs`
  - Run migration and verify tables exist

  **Must NOT do**:
  - Do NOT use UUID (keep i32 IDs)
  - Do NOT add password reset fields (defer)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 (first task)
  - **Blocks**: T2, T3, T4, T7, T9, T11, T17
  - **Blocked By**: None

  **References**:
  - `backend/src/models/post.rs` — Entity definition pattern
  - `backend/src/migrations/` — Migration pattern
  - `backend/src/models/mod.rs` — Module declarations

  **Acceptance Criteria**:
  - [ ] `users` table exists with all columns
  - [ ] `refresh_tokens` table exists with foreign key
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Migration runs successfully
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo run -- migrate`
      2. Assert exit code 0
      3. Run `psql -c "\dt"` to list tables
      4. Assert `users` and `refresh_tokens` tables exist
    Expected Result: Tables created
    Evidence: .omo/evidence/task-1-migration.txt
  ```

  **Commit**: YES
  - Message: `feat(db): add users and refresh_tokens schema`
  - Files: `backend/src/migrations/`, `backend/src/models/`

- [x] 2. JWT Auth Middleware

  **What to do**:
  - Add `jsonwebtoken` and `octocrab` to `Cargo.toml`
  - Add JWT config fields to `config.rs`: `jwt_secret`, `jwt_access_expiry`, `jwt_refresh_expiry`, `github_client_id`, `github_client_secret`, `github_redirect_uri`
  - Create `middleware/jwt.rs` — JWT validation middleware
  - Token payload: `{ sub: i32, role: String, exp: usize }`
  - Create `middleware/auth_guard.rs` — role-based access guard
  - Update `middleware/mod.rs` to export new modules
  - Implement dual-auth: accept both ADMIN_KEY and JWT during transition

  **Must NOT do**:
  - Do NOT remove ADMIN_KEY support yet (dual-auth)
  - Do NOT implement rate limiting

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T1)
  - **Blocks**: T3, T4, T5, T6
  - **Blocked By**: T1 (users table)

  **References**:
  - `backend/src/middleware/auth.rs` — Current auth pattern
  - `backend/src/config.rs` — Config pattern
  - `backend/src/state.rs` — AppState structure

  **Acceptance Criteria**:
  - [ ] JWT token generation works
  - [ ] JWT token validation works
  - [ ] Role-based access guard works
  - [ ] Dual-auth (ADMIN_KEY + JWT) works
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: JWT generation and validation
    Tool: Bash
    Steps:
      1. Run `cargo test jwt` — unit tests for JWT
      2. Assert exit code 0
    Expected Result: JWT tests pass
    Evidence: .omo/evidence/task-2-jwt.txt
  ```

  **Commit**: YES
  - Message: `feat(auth): add JWT middleware with role-based access`
  - Files: `backend/Cargo.toml`, `backend/src/config.rs`, `backend/src/middleware/`

- [x] 3. Auth Handlers (Register/Login/Refresh)

  **What to do**:
  - Create `handlers/auth.rs` with endpoints:
    - `POST /api/v1/auth/register` — 注册（返回 user + tokens）
    - `POST /api/v1/auth/login` — 登录（返回 user + tokens）
    - `POST /api/v1/auth/refresh` — 刷新 access token
    - `GET /api/v1/auth/me` — 获取当前用户信息
  - Create `routes/auth.rs` for route nesting
  - Register in `main.rs`
  - Add utoipa annotations
  - Use bcrypt for password hashing

  **Must NOT do**:
  - Do NOT implement password reset
  - Do NOT implement user management admin panel

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T2, T4)
  - **Blocks**: T5, T25
  - **Blocked By**: T2 (JWT middleware)

  **References**:
  - `backend/src/handlers/posts.rs` — Handler pattern
  - `backend/src/routes/subscribers.rs` — Route nesting pattern

  **Acceptance Criteria**:
  - [ ] `POST /api/v1/auth/register` returns 201 with user + tokens
  - [ ] `POST /api/v1/auth/login` returns 200 with user + tokens
  - [ ] `POST /api/v1/auth/refresh` returns new access token
  - [ ] `GET /api/v1/auth/me` returns current user
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: Registration and login flow
    Tool: Bash (curl)
    Steps:
      1. Register: `curl -X POST -d '{"email":"test@example.com","password":"pass123","name":"Test"}' http://localhost:8080/api/v1/auth/register`
      2. Assert status 201, response has user + access_token + refresh_token
      3. Login: `curl -X POST -d '{"email":"test@example.com","password":"pass123"}' http://localhost:8080/api/v1/auth/login`
      4. Assert status 200, response has tokens
    Expected Result: Registration and login work
    Evidence: .omo/evidence/task-3-auth-flow.txt
  ```

  **Commit**: YES
  - Message: `feat(auth): add register/login/refresh handlers`
  - Files: `backend/src/handlers/auth.rs`, `backend/src/routes/auth.rs`, `backend/src/main.rs`

- [x] 4. GitHub OAuth Handlers

  **What to do**:
  - Create GitHub OAuth endpoints:
    - `GET /api/v1/auth/github` — 重定向到 GitHub 授权页面
    - `GET /api/v1/auth/github/callback` — 处理回调，创建/关联账户，返回 tokens
  - 使用 `octocrab` crate 调用 GitHub API
  - 实现账号关联逻辑：相同邮箱自动关联
  - 添加 PKCE flow 安全性

  **Must NOT do**:
  - Do NOT support other OAuth providers (GitHub only)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T2, T3)
  - **Blocks**: T5
  - **Blocked By**: T2 (JWT middleware)

  **References**:
  - `backend/src/handlers/auth.rs` — Auth handler pattern
  - GitHub OAuth docs

  **Acceptance Criteria**:
  - [ ] `GET /api/v1/auth/github` redirects to GitHub
  - [ ] `GET /api/v1/auth/github/callback` creates user + returns tokens
  - [ ] Same email auto-links accounts
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: GitHub OAuth flow
    Tool: Bash (curl)
    Steps:
      1. `curl -v http://localhost:8080/api/v1/auth/github`
      2. Assert 302 redirect to github.com/login/oauth/authorize
    Expected Result: Redirect works
    Evidence: .omo/evidence/task-4-github-oauth.txt
  ```

  **Commit**: YES
  - Message: `feat(auth): add GitHub OAuth handlers`
  - Files: `backend/src/handlers/auth.rs`, `backend/src/main.rs`

- [x] 5. Frontend Auth Composables + Login Page

  **What to do**:
  - 创建 `composables/useAuth.ts` — 注册/登录/刷新/获取当前用户
  - 创建 `views/Login.vue` — 登录+注册表单
  - 创建 `views/GitHubCallback.vue` — OAuth 回调处理
  - 更新 `lib/api.ts` — 使用 JWT token 替代 ADMIN_KEY
  - 更新 `stores/app.ts` — 存储用户信息和 token
  - 添加路由 `/login` 和 `/github/callback`

  **Must NOT do**:
  - Do NOT add password reset page
  - Do NOT add user profile page

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T6)
  - **Blocks**: T6, T20-T24
  - **Blocked By**: T3, T4

  **References**:
  - `frontend/src/composables/usePosts.ts` — Composable pattern
  - `frontend/src/views/Home.vue` — View pattern
  - `frontend/src/lib/api.ts` — API client
  - `frontend/src/stores/app.ts` — Store pattern

  **Acceptance Criteria**:
  - [ ] Login page renders with email/password form
  - [ ] Registration form works
  - [ ] GitHub OAuth button redirects correctly
  - [ ] JWT token stored in localStorage
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Login page renders
    Tool: Playwright
    Steps:
      1. Navigate to /login
      2. Assert email input visible
      3. Assert password input visible
      4. Assert login button visible
      5. Assert GitHub login button visible
    Expected Result: Login page renders correctly
    Evidence: .omo/evidence/task-5-login-page.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add login page and auth composables`
  - Files: `frontend/src/composables/useAuth.ts`, `frontend/src/views/Login.vue`, `frontend/src/views/GitHubCallback.vue`, `frontend/src/lib/api.ts`, `frontend/src/stores/app.ts`, `frontend/src/router/index.ts`

- [x] 6. Frontend Route Guards

  **What to do**:
  - 添加 Vue Router 路由守卫
  - 未登录用户重定向到 /login
  - 角色权限检查（admin-only 路由）
  - Token 过期自动刷新逻辑
  - 保护路由：/publish, /drafts, /backup, /comments, /import

  **Must NOT do**:
  - Do NOT add user management admin page

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T5)
  - **Blocks**: —
  - **Blocked By**: T5

  **References**:
  - `frontend/src/router/index.ts` — Router config
  - `frontend/src/composables/useAuth.ts` — Auth composable

  **Acceptance Criteria**:
  - [ ] 未登录访问 /publish 重定向到 /login
  - [ ] 登录后可以访问受保护路由
  - [ ] Token 过期自动刷新
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Route guard redirects
    Tool: Playwright
    Steps:
      1. Clear localStorage
      2. Navigate to /publish
      3. Assert redirect to /login
    Expected Result: Redirect works
    Evidence: .omo/evidence/task-6-route-guard.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add route guards and token refresh`
  - Files: `frontend/src/router/index.ts`

### Wave 2: Core Features (Version Control + Media + PR)

- [x] 7. Version Control DB Schema

  **What to do**:
  - 创建 migration `m20240101_000013_add_author_id_to_posts.rs` — 添加 `author_id INT REFERENCES users(id)` 到 posts 表
  - 创建 migration `m20240101_000014_create_post_revisions_table.rs` — Schema: `id SERIAL PRIMARY KEY, post_id INT NOT NULL REFERENCES posts(id) ON DELETE CASCADE, title VARCHAR(500), content TEXT, excerpt TEXT, cover_image VARCHAR(500), version INT NOT NULL, created_by INT REFERENCES users(id), created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
  - 创建 migration `m20240101_000015_add_version_to_posts.rs` — 添加 `current_version INT NOT NULL DEFAULT 1` 到 posts 表
  - 创建 SeaORM entity `models/post_revision.rs`
  - 更新 `models/post.rs` 添加 author_id 和 current_version 字段

  **Must NOT do**:
  - Do NOT implement branching (linear history only)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T9, T11)
  - **Blocks**: T8, T14
  - **Blocked By**: T1

  **References**:
  - `backend/src/models/post.rs` — Entity pattern
  - `backend/src/migrations/` — Migration pattern

  **Acceptance Criteria**:
  - [ ] `post_revisions` table exists
  - [ ] `posts.author_id` column exists
  - [ ] `posts.current_version` column exists
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Migration runs
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo run -- migrate`
      2. Assert exit code 0
    Expected Result: Migrations applied
    Evidence: .omo/evidence/task-7-migration.txt
  ```

  **Commit**: YES
  - Message: `feat(db): add version control schema`
  - Files: `backend/src/migrations/`, `backend/src/models/`

- [x] 8. Version Control Handlers

  **What to do**:
  - 创建 `handlers/revisions.rs`:
    - `POST /api/v1/posts/:id/revisions` — 创建版本快照
    - `GET /api/v1/posts/:id/revisions` — 获取版本列表
    - `GET /api/v1/posts/:id/revisions/:rev_id` — 获取特定版本
    - `POST /api/v1/posts/:id/revisions/:rev_id/rollback` — 回滚到指定版本
    - `GET /api/v1/posts/:id/diff?from=&to=` — 版本 diff
  - 使用 `similar` crate 生成 unified diff
  - 回滚创建新版本（不破坏历史）
  - 自动保存版本：编辑文章时自动创建 revision

  **Must NOT do**:
  - Do NOT implement branching
  - Do NOT implement merge conflicts

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T10, T12, T13)
  - **Blocks**: T14, T26
  - **Blocked By**: T7

  **References**:
  - `backend/src/handlers/posts.rs` — Handler pattern
  - `similar` crate docs

  **Acceptance Criteria**:
  - [ ] `POST /api/v1/posts/:id/revisions` creates revision
  - [ ] `GET /api/v1/posts/:id/revisions` returns list
  - [ ] `POST /api/v1/posts/:id/revisions/:rev_id/rollback` restores content
  - [ ] `GET /api/v1/posts/:id/diff` returns diff
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: Version control flow
    Tool: Bash (curl)
    Steps:
      1. Create post
      2. Create revision: `curl -X POST -H "Auth..." http://localhost:8080/api/v1/posts/1/revisions`
      3. Update post content
      4. Create another revision
      5. Get diff: `curl http://localhost:8080/api/v1/posts/1/diff?from=1&to=2`
      6. Rollback: `curl -X POST http://localhost:8080/api/v1/posts/1/revisions/1/rollback`
      7. Assert post content restored
    Expected Result: Full version control flow works
    Evidence: .omo/evidence/task-8-version-control.txt
  ```

  **Commit**: YES
  - Message: `feat(version): add revision handlers with diff and rollback`
  - Files: `backend/src/handlers/revisions.rs`, `backend/src/routes/revisions.rs`, `backend/src/main.rs`

- [x] 9. Media DB Schema + StorageAdapter Trait

  **What to do**:
  - 创建 migration `m20240101_000017_create_media_table.rs` — Schema: `id SERIAL PRIMARY KEY, filename VARCHAR(255) NOT NULL, original_name VARCHAR(255) NOT NULL, mime_type VARCHAR(100) NOT NULL, size_bytes BIGINT NOT NULL, path VARCHAR(500) NOT NULL, uploaded_by INT REFERENCES users(id), created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
  - 创建 SeaORM entity `models/media.rs`
  - 创建 `storage/mod.rs` — StorageAdapter trait:
    ```rust
    #[async_trait]
    pub trait StorageAdapter: Send + Sync {
        async fn upload(&self, key: &str, data: &[u8], content_type: &str) -> Result<String>;
        async fn delete(&self, key: &str) -> Result<()>;
        fn get_url(&self, key: &str) -> String;
    }
    ```
  - 创建 `storage/local.rs` — LocalStorage 实现
  - 更新 `models/mod.rs` 和 `lib.rs`

  **Must NOT do**:
  - Do NOT implement S3Storage (only trait + local)
  - Do NOT add image processing

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T7, T11)
  - **Blocks**: T10, T15
  - **Blocked By**: T1

  **References**:
  - `backend/src/models/post.rs` — Entity pattern
  - `backend/src/migrations/` — Migration pattern

  **Acceptance Criteria**:
  - [ ] `media` table exists
  - [ ] StorageAdapter trait defined
  - [ ] LocalStorage implementation works
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Media table created
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo run -- migrate`
      2. Assert `media` table exists
    Expected Result: Table created
    Evidence: .omo/evidence/task-9-media-table.txt
  ```

  **Commit**: YES
  - Message: `feat(media): add media schema and StorageAdapter trait`
  - Files: `backend/src/migrations/`, `backend/src/models/media.rs`, `backend/src/storage/`

- [x] 10. Media Upload Handlers

  **What to do**:
  - 创建 `handlers/media.rs`:
    - `POST /api/v1/media` — 上传文件（multipart/form-data）
    - `GET /api/v1/media` — 获取媒体列表（分页）
    - `DELETE /api/v1/media/:id` — 删除文件
  - 使用 StorageAdapter 存储文件
  - 生成唯一文件名（UUID）
  - 验证文件类型和大小

  **Must NOT do**:
  - Do NOT add image processing/thumbnails
  - Do NOT add video transcoding

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T8, T12, T13)
  - **Blocks**: T15
  - **Blocked By**: T9

  **References**:
  - `backend/src/handlers/images.rs` — Current image upload pattern
  - `backend/src/handlers/videos.rs` — Current video upload pattern

  **Acceptance Criteria**:
  - [ ] `POST /api/v1/media` uploads file and returns metadata
  - [ ] `GET /api/v1/media` returns paginated list
  - [ ] `DELETE /api/v1/media/:id` removes file
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: Upload and list media
    Tool: Bash (curl)
    Steps:
      1. Upload: `curl -X POST -H "Auth..." -F "file=@test.jpg" http://localhost:8080/api/v1/media`
      2. Assert status 201, response has id, url, filename
      3. List: `curl -H "Auth..." http://localhost:8080/api/v1/media`
      4. Assert contains uploaded file
    Expected Result: Upload works
    Evidence: .omo/evidence/task-10-media-upload.txt
  ```

  **Commit**: YES
  - Message: `feat(media): add media upload handlers`
  - Files: `backend/src/handlers/media.rs`, `backend/src/routes/media.rs`, `backend/src/main.rs`

- [x] 11. PR Redesign DB Schema

  **What to do**:
  - 创建 migration `m20240101_000016_redesign_pull_requests_table.rs`
  - 添加字段: `fragments JSONB`（存储片段替换数据）, `user_id INT REFERENCES users(id)`, `message TEXT`
  - 创建新表 `pr_comments` — 内联评论: `id SERIAL PRIMARY KEY, pull_request_id INT REFERENCES pull_requests(id) ON DELETE CASCADE, fragment_index INT, line INT, content TEXT NOT NULL, user_id INT REFERENCES users(id), created_at TIMESTAMPTZ`
  - 更新 `models/pull_request.rs`

  **Must NOT do**:
  - Do NOT drop existing PR data (backward compat)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T7, T9)
  - **Blocks**: T12, T13, T16
  - **Blocked By**: T1

  **References**:
  - `backend/src/models/pull_request.rs` — Current PR model
  - `backend/src/migrations/` — Migration pattern

  **Acceptance Criteria**:
  - [ ] `pull_requests.fragments` column exists (JSONB)
  - [ ] `pr_comments` table exists
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: PR schema updated
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo run -- migrate`
      2. Assert `pr_comments` table exists
    Expected Result: Migration applied
    Evidence: .omo/evidence/task-11-pr-schema.txt
  ```

  **Commit**: YES
  - Message: `feat(db): redesign PR schema with fragments`
  - Files: `backend/src/migrations/`, `backend/src/models/`

- [x] 12. PR Fragment Mapping Algorithm

  **What to do**:
  - 创建 `utils/fragment_mapper.rs`:
    - 渲染后选中的文字位置 → 源文本位置映射
    - 使用字符偏移量存储位置
    - 处理边界情况：部分单词、跨段落、代码块
    - 验证片段位置是否有效（未越界）
  - 创建 `utils/diff_engine.rs`:
    - 使用 `similar` crate 生成 diff
    - 应用片段替换到源文本
    - 冲突检测（重叠片段）
  - 编写全面的单元测试

  **Must NOT do**:
  - Do NOT implement real-time mapping (batch only)

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T8, T10, T13)
  - **Blocks**: T13, T16
  - **Blocked By**: T11

  **References**:
  - `similar` crate docs
  - Text diff algorithms

  **Acceptance Criteria**:
  - [ ] Fragment mapping works for simple text
  - [ ] Fragment mapping works for multi-paragraph
  - [ ] Fragment validation catches out-of-bounds
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: Fragment mapping accuracy
    Tool: Bash
    Steps:
      1. Run `cargo test fragment_mapper`
      2. Assert all tests pass
    Expected Result: Mapping algorithm works
    Evidence: .omo/evidence/task-12-fragment-mapping.txt
  ```

  **Commit**: YES
  - Message: `feat(utils): add fragment mapping algorithm`
  - Files: `backend/src/utils/`

- [x] 13. PR Handlers (Create/Apply/Inline Comments)

  **What to do**:
  - 重构 `handlers/pulls.rs`:
    - `POST /api/v1/posts/:id/pulls` — 创建 PR（包含 fragments + message）
    - `GET /api/v1/posts/:id/pulls` — 获取 PR 列表
    - `GET /api/v1/pulls/:id` — 获取 PR 详情（包含 fragments）
    - `POST /api/v1/pulls/:id/apply` — 应用 PR（合并 fragments 到文章）
    - `POST /api/v1/pulls/:id/comments` — 添加内联评论
    - `GET /api/v1/pulls/:id/comments` — 获取评论列表
  - 应用 PR 时自动创建 revision
  - 验证 fragments 位置有效性

  **Must NOT do**:
  - Do NOT implement PR approval workflow
  - Do NOT implement multi-file PRs

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T8, T10, T12)
  - **Blocks**: T16
  - **Blocked By**: T12

  **References**:
  - `backend/src/handlers/pulls.rs` — Current PR handler
  - `backend/src/handlers/posts.rs` — Handler pattern

  **Acceptance Criteria**:
  - [ ] `POST /api/v1/posts/:id/pulls` creates PR with fragments
  - [ ] `POST /api/v1/pulls/:id/apply` merges fragments into post
  - [ ] Applying PR creates revision
  - [ ] Inline comments work
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: PR create and apply
    Tool: Bash (curl)
    Steps:
      1. Create PR: `curl -X POST -H "Auth..." -d '{"fragments":[...],"message":"Fix typo"}' http://localhost:8080/api/v1/posts/1/pulls`
      2. Assert status 201
      3. Apply: `curl -X POST -H "Auth..." http://localhost:8080/api/v1/pulls/1/apply`
      4. Assert post content updated
      5. Assert revision created
    Expected Result: PR flow works
    Evidence: .omo/evidence/task-13-pr-apply.txt
  ```

  **Commit**: YES
  - Message: `feat(pr): redesign PR with fragment selection and apply`
  - Files: `backend/src/handlers/pulls.rs`, `backend/src/main.rs`

- [x] 14. Frontend Version Control UI

  **What to do**:
  - 创建 `composables/useRevisions.ts` — 版本 API 调用
  - 在 `PostDetail.vue` 添加版本历史面板
  - 显示版本列表（时间、作者、版本号）
  - Diff 视图（使用 `diff` 库渲染 unified diff）
  - 回滚按钮 + 确认对话框

  **Must NOT do**:
  - Do NOT add分支可视化

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T15, T16)
  - **Blocks**: T27
  - **Blocked By**: T8

  **References**:
  - `frontend/src/views/PostDetail.vue` — Post detail page
  - `frontend/src/composables/usePosts.ts` — Composable pattern

  **Acceptance Criteria**:
  - [ ] 版本历史面板显示
  - [ ] Diff 视图渲染正确
  - [ ] 回滚功能工作
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Version history panel
    Tool: Playwright
    Steps:
      1. Navigate to post detail
      2. Click "版本历史" button
      3. Assert version list visible
      4. Click "查看 Diff"
      5. Assert diff view renders
    Expected Result: Version UI works
    Evidence: .omo/evidence/task-14-version-ui.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add version history and diff view`
  - Files: `frontend/src/composables/useRevisions.ts`, `frontend/src/views/PostDetail.vue`

- [x] 15. Frontend Media Manager UI

  **What to do**:
  - 创建 `composables/useMedia.ts` — 媒体 API 调用
  - 创建 `views/Media.vue` — 媒体管理页面
  - 文件上传组件（拖拽 + 点击）
  - 媒体列表（网格/列表视图）
  - 删除确认
  - 添加路由 `/media`

  **Must NOT do**:
  - Do NOT add image cropping/editing

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T14, T16)
  - **Blocks**: T27
  - **Blocked By**: T10

  **References**:
  - `frontend/src/views/Import.vue` — File upload pattern
  - `frontend/src/composables/usePosts.ts` — Composable pattern

  **Acceptance Criteria**:
  - [ ] 媒体管理页面渲染
  - [ ] 文件上传工作
  - [ ] 媒体列表显示
  - [ ] 删除功能工作
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Media upload
    Tool: Playwright
    Steps:
      1. Navigate to /media
      2. Upload a file
      3. Assert file appears in list
    Expected Result: Upload works
    Evidence: .omo/evidence/task-15-media-upload.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add media manager page`
  - Files: `frontend/src/composables/useMedia.ts`, `frontend/src/views/Media.vue`, `frontend/src/router/index.ts`

- [x] 16. Frontend PR Redesign UI

  **What to do**:
  - 重构 `components/PullRequest.vue`:
    - 文章渲染后选中文字创建 PR
    - 片段高亮显示
    - 内联评论
    - Apply 按钮（admin only）
  - 创建 `composables/usePullRequests.ts` — PR API 调用
  - 在 `PostDetail.vue` 集成 PR 功能

  **Must NOT do**:
  - Do NOT add实时协作

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T14, T15)
  - **Blocks**: T27
  - **Blocked By**: T13

  **References**:
  - `frontend/src/components/PullRequest.vue` — Current PR component
  - `frontend/src/views/PostDetail.vue` — Post detail page

  **Acceptance Criteria**:
  - [ ] 选中文字创建 PR
  - [ ] 片段高亮显示
  - [ ] 内联评论工作
  - [ ] Apply 功能工作
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: PR creation from text selection
    Tool: Playwright
    Steps:
      1. Navigate to post detail
      2. Select some text
      3. Click "创建 PR"
      4. Fill in replacement text
      5. Submit
      6. Assert PR created
    Expected Result: PR creation works
    Evidence: .omo/evidence/task-16-pr-creation.png
  ```

  **Commit**: YES
  - Message: `feat(ui): redesign PR with fragment selection`
  - Files: `frontend/src/components/PullRequest.vue`, `frontend/src/composables/usePullRequests.ts`, `frontend/src/views/PostDetail.vue`

### Wave 3: Integration (TODO + Homepage)

- [x] 17. TODO Subscription DB Schema

  **What to do**:
  - 创建 migration `m20240101_000018_create_todo_subscriptions_table.rs`
  - Schema: `id SERIAL PRIMARY KEY, todo_item_id INT NOT NULL REFERENCES todo_items(id) ON DELETE CASCADE, user_id INT REFERENCES users(id), email VARCHAR(255), created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), UNIQUE(todo_item_id, user_id), UNIQUE(todo_item_id, email)`
  - 创建 SeaORM entity `models/todo_subscription.rs`
  - 更新 `models/mod.rs`

  **Must NOT do**:
  - Do NOT add优先级/截止日期等 TODO 扩展字段

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T20, T21, T22)
  - **Blocks**: T18, T19, T23, T24
  - **Blocked By**: T1

  **References**:
  - `backend/src/models/todo_item.rs` — Current TODO model
  - `backend/src/migrations/` — Migration pattern

  **Acceptance Criteria**:
  - [ ] `todo_subscriptions` table exists
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Migration runs
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo run -- migrate`
      2. Assert `todo_subscriptions` table exists
    Expected Result: Table created
    Evidence: .omo/evidence/task-17-migration.txt
  ```

  **Commit**: YES
  - Message: `feat(db): add TODO subscriptions schema`
  - Files: `backend/src/migrations/`, `backend/src/models/`

- [x] 18. TODO Handlers (List/Subscribe/Complete)

  **What to do**:
  - 创建 `handlers/todos.rs`:
    - `GET /api/v1/todos` — 获取 TODO 列表（admin 看全部，visitor 看已发布文章的）
    - `POST /api/v1/todos/:id/subscribe` — 订阅 TODO
    - `DELETE /api/v1/todos/:id/subscribe` — 取消订阅
    - `POST /api/v1/todos/:id/complete` — 标记 TODO 完成（admin only）
  - 创建 `routes/todos.rs`
  - 注册到 `main.rs`

  **Must NOT do**:
  - Do NOT添加 TODO 分配/优先级功能

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T19, T20-T22)
  - **Blocks**: T19, T23
  - **Blocked By**: T17

  **References**:
  - `backend/src/handlers/posts.rs` — Handler pattern
  - `backend/src/models/todo_item.rs` — TODO model

  **Acceptance Criteria**:
  - [ ] `GET /api/v1/todos` returns TODO list
  - [ ] `POST /api/v1/todos/:id/subscribe` subscribes user
  - [ ] `POST /api/v1/todos/:id/complete` marks complete
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: TODO subscribe and complete
    Tool: Bash (curl)
    Steps:
      1. Subscribe: `curl -X POST -H "Auth..." http://localhost:8080/api/v1/todos/1/subscribe`
      2. Assert status 201
      3. Complete: `curl -X POST -H "Auth..." http://localhost:8080/api/v1/todos/1/complete`
      4. Assert status 200
    Expected Result: TODO flow works
    Evidence: .omo/evidence/task-18-todo-flow.txt
  ```

  **Commit**: YES
  - Message: `feat(todo): add TODO management handlers`
  - Files: `backend/src/handlers/todos.rs`, `backend/src/routes/todos.rs`, `backend/src/main.rs`

- [x] 19. TODO Email Notification

  **What to do**:
  - 更新 `tasks/email.rs`:
    - 添加 `notify_todo_subscribers()` 函数
    - 当 TODO 标记完成时，发送邮件给所有订阅者
    - 邮件模板：TODO 标题、文章链接、完成时间
  - 在 `handlers/todos.rs` 的 complete 处理中调用通知
  - 使用现有的 lettre 配置

  **Must NOT do**:
  - Do NOT添加邮件队列（直接发送）
  - Do NOT添加重试逻辑

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T18, T20-T22)
  - **Blocks**: T24
  - **Blocked By**: T18

  **References**:
  - `backend/src/tasks/email.rs` — Current email pattern

  **Acceptance Criteria**:
  - [ ] TODO 完成时发送邮件
  - [ ] 邮件包含 TODO 详情
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: Email notification sent
    Tool: Bash
    Steps:
      1. Subscribe to TODO
      2. Complete TODO
      3. Check SMTP logs for email
    Expected Result: Email sent
    Evidence: .omo/evidence/task-19-email.txt
  ```

  **Commit**: YES
  - Message: `feat(todo): add email notification for TODO completion`
  - Files: `backend/src/tasks/email.rs`, `backend/src/handlers/todos.rs`

- [x] 20. Homepage Hero Animation

  **What to do**:
  - 创建 `components/HeroSection.vue`:
    - 大标题 "ZYBlog"
    - 动态文字效果（CSS+JS 循环显示不同文字）
    - 类似 mimo.xiaomi.com 的 "HELLO, I'M MiMo" 效果
    - 响应式设计
  - 在 `Home.vue` 集成 Hero 区域

  **Must NOT do**:
  - Do NOT使用 Three.js/GSAP/Canvas（纯 CSS+JS）
  - Do NOT添加 3D 效果

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T17-T19, T21, T22)
  - **Blocks**: T27
  - **Blocked By**: T5

  **References**:
  - mimo.xiaomi.com — Hero 动画效果
  - `frontend/src/views/Home.vue` — 主页

  **Acceptance Criteria**:
  - [ ] Hero 区域渲染
  - [ ] 文字动画循环
  - [ ] 响应式布局
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Hero animation renders
    Tool: Playwright
    Steps:
      1. Navigate to /
      2. Assert hero section visible
      3. Assert animated text visible
      4. Wait 3s, assert text changed
    Expected Result: Animation works
    Evidence: .omo/evidence/task-20-hero.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add hero animation section`
  - Files: `frontend/src/components/HeroSection.vue`, `frontend/src/views/Home.vue`

- [x] 21. Homepage Article Card Grid

  **What to do**:
  - 创建 `components/ArticleCard.vue`:
    - 封面图片
    - 标题 + 摘要
    - 标签 badges
    - 发布时间
    - 悬停效果
  - 在 `Home.vue` 添加卡片网格布局（1/2/3 列响应式）
  - 精选文章筛选逻辑

  **Must NOT do**:
  - Do NOT添加无限滚动（保持分页）

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T17-T20, T22)
  - **Blocks**: T27
  - **Blocked By**: T5

  **References**:
  - `frontend/src/views/Home.vue` — 当前文章列表
  - `frontend/src/views/PostDetail.vue` — 文章详情

  **Acceptance Criteria**:
  - [ ] 卡片网格渲染
  - [ ] 响应式布局（1/2/3 列）
  - [ ] 悬停效果
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Card grid renders
    Tool: Playwright
    Steps:
      1. Navigate to /
      2. Assert article cards visible
      3. Resize to mobile, assert 1 column
      4. Resize to desktop, assert 3 columns
    Expected Result: Grid responsive
    Evidence: .omo/evidence/task-21-card-grid.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add article card grid`
  - Files: `frontend/src/components/ArticleCard.vue`, `frontend/src/views/Home.vue`

- [x] 22. Homepage Category Filter

  **What to do**:
  - 在主页添加标签/分类筛选栏
  - 点击标签筛选文章
  - 筛选状态 URL 同步（?tag=xxx）
  - "全部" 按钮重置筛选

  **Must NOT do**:
  - Do NOT添加搜索功能（已有）

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T17-T21)
  - **Blocks**: T27
  - **Blocked By**: T5

  **References**:
  - `frontend/src/views/Home.vue` — 当前标签筛选

  **Acceptance Criteria**:
  - [ ] 标签筛选栏显示
  - [ ] 点击标签筛选文章
  - [ ] URL 同步
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Category filter works
    Tool: Playwright
    Steps:
      1. Navigate to /
      2. Click a tag
      3. Assert articles filtered
      4. Assert URL contains ?tag=
    Expected Result: Filter works
    Evidence: .omo/evidence/task-22-category-filter.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add category filter on homepage`
  - Files: `frontend/src/views/Home.vue`

- [x] 23. Frontend TODO Management UI

  **What to do**:
  - 创建 `views/Todos.vue` — TODO 管理页面（admin）
  - 显示所有 TODO 列表（按文章分组）
  - 标记完成按钮
  - 筛选：全部/未完成/已完成
  - 添加路由 `/todos`

  **Must NOT do**:
  - Do NOT添加 TODO 编辑功能

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T24)
  - **Blocks**: T27
  - **Blocked By**: T18

  **References**:
  - `frontend/src/views/Comments.vue` — 管理页面模式

  **Acceptance Criteria**:
  - [ ] TODO 管理页面渲染
  - [ ] 标记完成功能工作
  - [ ] 筛选功能工作
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: TODO management
    Tool: Playwright
    Steps:
      1. Navigate to /todos
      2. Assert TODO list visible
      3. Click "完成" button
      4. Assert TODO marked complete
    Expected Result: Management works
    Evidence: .omo/evidence/task-23-todo-management.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add TODO management page`
  - Files: `frontend/src/views/Todos.vue`, `frontend/src/router/index.ts`

- [x] 24. Frontend TODO Subscription UI

  **What to do**:
  - 在 `PostDetail.vue` 添加 TODO 订阅按钮
  - 显示 TODO 列表（从文章内容提取）
  - 订阅/取消订阅
  - 已订阅状态显示
  - 邮箱输入（未登录用户）

  **Must NOT do**:
  - Do NOT添加订阅管理页面

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T23)
  - **Blocks**: T27
  - **Blocked By**: T19

  **References**:
  - `frontend/src/views/PostDetail.vue` — 文章详情页
  - `frontend/src/components/TodoSubscribe.vue` — 现有 TODO 组件

  **Acceptance Criteria**:
  - [ ] TODO 订阅按钮显示
  - [ ] 订阅/取消订阅工作
  - [ ] 已订阅状态正确
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: TODO subscription
    Tool: Playwright
    Steps:
      1. Navigate to post detail
      2. Click "订阅 TODO"
      3. Assert subscription confirmed
    Expected Result: Subscription works
    Evidence: .omo/evidence/task-24-todo-subscription.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add TODO subscription on post detail`
  - Files: `frontend/src/views/PostDetail.vue`, `frontend/src/components/TodoSubscribe.vue`

### Wave 4: Polish + Final

- [x] 25. Auth Migration (ADMIN_KEY → JWT Transition)

  **What to do**:
  - 更新 `middleware/auth.rs` — 实现双认证（ADMIN_KEY + JWT）
  - 创建 admin 用户（从 ADMIN_KEY 环境变量迁移）
  - 更新 `config.rs` — 添加 JWT 相关配置
  - 更新前端 `lib/api.ts` — 优先使用 JWT，fallback 到 ADMIN_KEY
  - 文档：迁移指南

  **Must NOT do**:
  - Do NOT移除 ADMIN_KEY 支持（保持双认证）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with T27, T28)
  - **Blocks**: T26
  - **Blocked By**: T3

  **References**:
  - `backend/src/middleware/auth.rs` — Current auth
  - `backend/src/config.rs` — Config pattern

  **Acceptance Criteria**:
  - [ ] 双认证中间件工作
  - [ ] Admin 用户创建成功
  - [ ] JWT 和 ADMIN_KEY 都能认证
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: Dual auth works
    Tool: Bash (curl)
    Steps:
      1. Login to get JWT
      2. Use JWT to create post — assert 201
      3. Use ADMIN_KEY to create post — assert 201
    Expected Result: Both auth methods work
    Evidence: .omo/evidence/task-25-dual-auth.txt
  ```

  **Commit**: YES
  - Message: `feat(auth): implement dual auth migration`
  - Files: `backend/src/middleware/auth.rs`, `backend/src/config.rs`

- [x] 26. Blog Editing Integration

  **What to do**:
  - 更新 `handlers/posts.rs`:
    - 编辑文章时自动创建 revision
    - 更新 `update_post` 处理逻辑
  - 更新 `Publish.vue`:
    - 编辑模式（从文章列表进入）
    - 自动保存草稿
    - 版本历史快捷入口
  - 更新路由支持 `/publish/:id` 编辑模式

  **Must NOT do**:
  - Do NOT添加实时协作编辑

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with T25, T27)
  - **Blocks**: T27
  - **Blocked By**: T8, T25

  **References**:
  - `backend/src/handlers/posts.rs` — 更新逻辑
  - `frontend/src/views/Publish.vue` — 发布页面

  **Acceptance Criteria**:
  - [ ] 编辑文章自动创建 revision
  - [ ] `/publish/:id` 加载现有文章
  - [ ] 自动保存工作
  - [ ] `cargo test` passes
  - [ ] `npm run build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Edit post creates revision
    Tool: Bash (curl)
    Steps:
      1. Create post
      2. Edit post: `curl -X PUT -H "Auth..." -d '{"title":"Updated"}' http://localhost:8080/api/v1/posts/1`
      3. Get revisions: `curl http://localhost:8080/api/v1/posts/1/revisions`
      4. Assert 2 revisions exist
    Expected Result: Auto-revision works
    Evidence: .omo/evidence/task-26-auto-revision.txt
  ```

  **Commit**: YES
  - Message: `feat(edit): integrate auto-revision on post edit`
  - Files: `backend/src/handlers/posts.rs`, `frontend/src/views/Publish.vue`, `frontend/src/router/index.ts`

- [x] 27. E2e Tests (Playwright)

  **What to do**:
  - 创建 e2e 测试文件:
    - `e2e/auth.spec.ts` — 注册/登录/OAuth 流程
    - `e2e/version-control.spec.ts` — 版本控制流程
    - `e2e/pull-requests.spec.ts` — PR 创建/应用流程
    - `e2e/media.spec.ts` — 媒体上传/管理
    - `e2e/homepage.spec.ts` — 主页功能
    - `e2e/todos.spec.ts` — TODO 管理/订阅
  - 更新 `playwright.config.ts`

  **Must NOT do**:
  - Do NOT测试每个边缘情况（关键路径 only）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (after T14-T16, T20-T24)
  - **Blocks**: T28
  - **Blocked By**: T14-T16, T20-T24

  **References**:
  - `frontend/e2e/` — 现有测试模式

  **Acceptance Criteria**:
  - [ ] 6 个测试文件创建
  - [ ] `npx playwright test` passes

  **QA Scenarios**:
  ```
  Scenario: E2e tests pass
    Tool: Bash
    Steps:
      1. Run `cd frontend && npx playwright test`
      2. Assert exit code 0
    Expected Result: All tests pass
    Evidence: .omo/evidence/task-27-e2e.txt
  ```

  **Commit**: YES
  - Message: `test(e2e): add Playwright tests for all features`
  - Files: `frontend/e2e/`

- [x] 28. Integration Testing + Bug Fixes

  **What to do**:
  - 运行完整测试套件: `cargo test` + `npm run test` + `npx playwright test`
  - 修复发现的 bug
  - 验证跨功能集成:
    - 用户认证 + 版本控制
    - PR + 版本控制
    - TODO + 邮件通知
    - 主页 + 文章列表

  **Must NOT do**:
  - Do NOT添加新功能（仅修复）

  **Recommended Agent Profile**:
  - **Category**: `deep`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (after T27)
  - **Blocks**: F1-F4
  - **Blocked By**: T27

  **References**:
  - 所有任务 QA 场景

  **Acceptance Criteria**:
  - [ ] `cargo test` passes
  - [ ] `npm run test` passes
  - [ ] `npx playwright test` passes
  - [ ] No known bugs

  **QA Scenarios**:
  ```
  Scenario: Full test suite
    Tool: Bash
    Steps:
      1. Run `cd backend && cargo test`
      2. Assert exit code 0
      3. Run `cd frontend && npm run test`
      4. Assert exit code 0
      5. Run `cd frontend && npx playwright test`
      6. Assert exit code 0
    Expected Result: All tests pass
    Evidence: .omo/evidence/task-28-full-suite.txt
  ```

  **Commit**: YES (if fixes needed)
  - Message: `fix(integration): resolve bugs from testing`

---

## Final Verification Wave

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists. For each "Must NOT Have": search codebase for forbidden patterns. Check evidence files exist. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo test` + `npm run test`. Review all changed files for: `unwrap()` in handlers, missing utoipa annotations, `any` types in TypeScript, unused imports. Check AI slop patterns.
  Output: `Backend Tests [PASS/FAIL] | Frontend Tests [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high` (+ `playwright` skill)
  Start from clean state. Execute EVERY QA scenario from EVERY task. Test cross-task integration. Test edge cases.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff. Verify 1:1 — everything in spec was built, nothing beyond spec. Check "Must NOT do" compliance. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

| Wave | Commit | Files |
|------|--------|-------|
| 1 | `feat(auth): add user system with JWT + GitHub OAuth` | backend/src/ |
| 1 | `feat(ui): add login page and route guards` | frontend/src/ |
| 2 | `feat(version): add post revision control` | backend/src/, frontend/src/ |
| 2 | `feat(media): add media management with StorageAdapter` | backend/src/, frontend/src/ |
| 2 | `feat(pr): redesign PR with fragment selection` | backend/src/, frontend/src/ |
| 3 | `feat(todo): add TODO management and subscriptions` | backend/src/, frontend/src/ |
| 3 | `feat(home): redesign homepage with hero animation` | frontend/src/ |
| 4 | `feat(auth): migrate from ADMIN_KEY to JWT` | backend/src/ |
| 4 | `test(e2e): add Playwright tests for all features` | frontend/e2e/ |

---

## Success Criteria

### Verification Commands
```bash
# Backend
cd backend && cargo test      # Expected: all tests pass
cd backend && cargo build     # Expected: no errors

# Frontend
cd frontend && npm run test   # Expected: all tests pass
cd frontend && npm run build  # Expected: build succeeds

# E2e
cd frontend && npx playwright test  # Expected: all e2e tests pass
```

### Final Checklist
- [ ] 用户注册/登录/JWT 工作正常
- [ ] GitHub OAuth 登录工作正常
- [ ] 三种角色权限正确
- [ ] 文章版本控制（保存+回滚+diff）工作正常
- [ ] PR 片段选中+应用工作正常
- [ ] 媒体文件上传+管理工作正常
- [ ] 主页 Hero 动画+卡片网格+分类筛选工作正常
- [ ] TODO 管理+订阅+邮件通知工作正常
- [ ] 所有测试通过
- [ ] 无 scope creep
