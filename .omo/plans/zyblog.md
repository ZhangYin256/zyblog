# ZYBlog - 个人博客系统 MVP 开发计划

## TL;DR

> **Quick Summary**: 构建一个支持备忘录式发布、#todo邮件订阅、Obsidian导入的单用户个人博客系统，采用 Vue 3 + Axum (Rust) + PostgreSQL 技术栈。
> 
> **Deliverables**:
> - 完整的后端 API（Axum + SeaORM）
> - 响应式前端应用（Vue 3 + Naive UI）
> - Docker Compose 开发环境
> - Git 仓库初始化
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: Task 1 → Task 4 → Task 8 → Task 12 → Task 16

---

## Context

### Original Request
基于 `需求说明.md` 文档，构建个人博客系统。用户此前使用WordPress但因发布流程繁琐而废弃，希望创建一个集"便捷发布"与"丰富功能"于一体的个人博客系统。

### Interview Summary
**Key Discussions**:
- 技术栈选择：Vue 3 + Axum (Rust) + PostgreSQL + SeaORM
- MVP范围：单用户、备忘录式发布、#todo邮件订阅、Obsidian导入
- UI风格：简约清爽，参考手机自带备忘录
- 部署：本地Docker开发，后续云服务器

**Research Findings**:
- Axum + SeaORM：成熟的Rust Web开发栈，SeaORM 2.0 RC可用
- Vue 3 + Naive UI：组件库设计简洁，`md-editor-v3`支持Markdown编辑
- 测试：Vitest + Vue Test Utils (前端)，Rust标准测试 (后端)

### Metis Review
**Identified Gaps** (addressed):
- MVP范围对齐：确认#todo邮件订阅和Obsidian导入属于MVP
- 认证机制：使用静态Bearer Token而非JWT
- Git仓库初始化：明确包含在MVP中
- 图片处理：本地存储，5MB限制，上传时压缩

---

## Work Objectives

### Core Objective
构建一个单用户个人博客系统，支持备忘录式发布、#todo标签邮件订阅、Obsidian文件导入，采用简约清爽的UI设计。

### Concrete Deliverables
- 后端API服务（Axum + SeaORM + PostgreSQL）
- 前端Web应用（Vue 3 + Naive UI，响应式布局）
- Docker Compose开发环境
- Git仓库初始化

### Definition of Done
- [ ] `docker-compose up` 启动完整开发环境
- [ ] 移动端可访问并使用备忘录式发布
- [ ] PC端可访问并浏览文章
- [ ] #todo标签可高亮显示并支持邮件订阅
- [ ] Obsidian .md文件可上传导入
- [ ] 所有测试通过：`cargo test` 和 `vitest run`

### Must Have
- 单用户博客系统（无注册/登录）
- 备忘录式发布（标题+正文+图片，极简界面）
- #todo标签高亮 + 邮件订阅功能
- Obsidian文件上传导入
- 响应式布局（移动端/PC端适配）
- 管理员认证（环境变量Bearer Token）
- 图片上传（本地存储，5MB限制）
- 手动数据导出

### Must NOT Have (Guardrails)
- ❌ 多用户系统（注册/登录）
- ❌ JWT认证（使用静态Bearer Token）
- ❌ PR式互动功能
- ❌ 省流模式
- ❌ 树状结构呈现
- ❌ 痕迹记录
- ❌ 移动端原生App
- ❌ 视频上传支持
- ❌ 自动备份

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** - ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (greenfield)
- **Automated tests**: Tests-after
- **Framework**: Vitest (前端), Rust standard tests (后端)
- **Process**: 先实现功能，再补测试

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.omo/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Frontend/UI**: Use Playwright - Navigate, interact, assert DOM, screenshot
- **API/Backend**: Use Bash (curl) - Send requests, assert status + response fields
- **Integration**: Use Docker Compose - Full stack testing

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately - foundation):
├── Task 1: Git仓库初始化 [quick]
├── Task 2: 后端项目骨架 [quick]
├── Task 3: 前端项目骨架 [quick]
├── Task 4: 数据库设计与迁移 [quick]
└── Task 5: Docker Compose环境 [quick]

Wave 2 (After Wave 1 - core backend):
├── Task 6: 文章CRUD API (depends: 4) [unspecified-high]
├── Task 7: 图片上传API (depends: 4) [unspecified-high]
├── Task 8: 管理员认证中间件 (depends: 2) [unspecified-high]
└── Task 9: #todo解析与邮件服务 (depends: 2, 4) [deep]

Wave 3 (After Wave 2 - core frontend):
├── Task 10: 响应式布局框架 (depends: 3) [visual-engineering]
├── Task 11: 备忘录式发布界面 (depends: 10) [visual-engineering]
├── Task 12: 文章列表与详情页 (depends: 10) [visual-engineering]
├── Task 13: Obsidian导入界面 (depends: 10) [visual-engineering]
└── Task 14: #todo订阅界面 (depends: 10) [visual-engineering]

Wave 4 (After Wave 3 - integration):
├── Task 15: 前后端联调 (depends: 6,7,8,11,12,13,14) [unspecified-high]
├── Task 16: 测试补充 (depends: 15) [unspecified-high]
└── Task 17: 文档与导出功能 (depends: 15) [writing]

Wave FINAL (After ALL tasks — 4 parallel reviews):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)
-> Present results -> Get explicit user okay

Critical Path: Task 1 → Task 4 → Task 8 → Task 15 → Task 16 → F1-F4 → user okay
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 5 (Waves 1 & 3)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|------------|--------|
| 1 | - | 2,3,4,5 |
| 2 | 1 | 6,7,8,9 |
| 3 | 1 | 10,11,12,13,14 |
| 4 | 1 | 6,7,9 |
| 5 | 1 | - |
| 6 | 2,4 | 15 |
| 7 | 2,4 | 15 |
| 8 | 2 | 15 |
| 9 | 2,4 | 15 |
| 10 | 3 | 11,12,13,14 |
| 11 | 10 | 15 |
| 12 | 10 | 15 |
| 13 | 10 | 15 |
| 14 | 10 | 15 |
| 15 | 6,7,8,9,11,12,13,14 | 16,17 |
| 16 | 15 | F1-F4 |
| 17 | 15 | F1-F4 |
| F1-F4 | 16,17 | user okay |

### Agent Dispatch Summary

- **Wave 1**: 5 tasks - T1-T5 → `quick`
- **Wave 2**: 4 tasks - T6-T9 → `unspecified-high` / `deep`
- **Wave 3**: 5 tasks - T10-T14 → `visual-engineering`
- **Wave 4**: 3 tasks - T15-T17 → `unspecified-high` / `writing`
- **FINAL**: 4 tasks - F1 → `oracle`, F2-F4 → `unspecified-high` / `deep`

---

## TODOs

- [x] 1. Git 仓库初始化

  **What to do**:
  - 在 `/home/zy/zyblog` 目录初始化 Git 仓库
  - 创建 `.gitignore` 文件（Rust, Node.js, Docker 常见忽略项）
  - 创建项目根目录 README.md
  - 首次提交：项目骨架文件

  **Must NOT do**:
  - 不要提交 `.env` 文件或敏感信息
  - 不要提交 `node_modules/` 或 `target/`

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 简单的 Git 初始化任务，无复杂逻辑
  - **Skills**: [`git-master`]
    - `git-master`: Git 操作需要专业指导

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4, 5)
  - **Blocks**: Tasks 2, 3, 4, 5
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - 无（greenfield 项目）

  **External References**:
  - GitHub gitignore templates: https://github.com/github/gitignore

  **Acceptance Criteria**:

  ```
  Scenario: Git 仓库初始化成功
    Tool: Bash
    Preconditions: /home/zy/zyblog 目录存在
    Steps:
      1. cd /home/zy/zyblog && git status
      2. ls -la .git/
      3. cat .gitignore | grep -c "target/\|node_modules/"
    Expected Result: git status 显示 "On branch main"，.gitignore 包含 Rust 和 Node.js 忽略项
    Evidence: .omo/evidence/task-1-git-init.txt

  Scenario: 首次提交成功
    Tool: Bash
    Preconditions: Git 仓库已初始化
    Steps:
      1. git log --oneline
      2. git status
    Expected Result: 至少 1 个 commit，working tree clean
    Evidence: .omo/evidence/task-1-first-commit.txt
  ```

  **Commit**: YES
  - Message: `feat(init): initialize git repository`
  - Files: `.gitignore`, `README.md`
  - Pre-commit: N/A

- [x] 2. 后端项目骨架搭建

  **What to do**:
  - 创建 Rust 项目结构（Axum + SeaORM）
  - 配置 `Cargo.toml` 依赖（axum, sea-orm, tokio, serde, etc.）
  - 创建目录结构：`src/handlers/`, `src/routes/`, `src/models/`, `src/tasks/`, `src/middleware/`
  - 创建基础文件：`src/main.rs`, `src/config.rs`, `src/error.rs`, `src/state.rs`
  - 实现健康检查端点 `GET /api/health`

  **Must NOT do**:
  - 不要实现具体业务逻辑（留给后续任务）
  - 不要创建数据库模型（Task 4）

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 项目脚手架搭建，结构清晰
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4, 5)
  - **Blocks**: Tasks 6, 7, 8, 9
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - 无（greenfield 项目）

  **External References**:
  - Axum 官方示例: https://github.com/tokio-rs/axum/tree/main/examples
  - SeaORM 快速入门: https://www.sea-ql.org/SeaORM/docs/getting-started/quick-start

  **Acceptance Criteria**:

  ```
  Scenario: 后端项目可编译
    Tool: Bash
    Preconditions: Rust 工具链已安装
    Steps:
      1. cd backend && cargo check
      2. cargo run --bin zyblog &
      3. curl http://localhost:8080/api/health
      4. kill %1
    Expected Result: cargo check 成功，health 端点返回 200
    Evidence: .omo/evidence/task-2-backend-skeleton.txt

  Scenario: 项目结构正确
    Tool: Bash
    Preconditions: 后端项目已创建
    Steps:
      1. ls -la backend/src/
      2. find backend/src -name "*.rs" | wc -l
    Expected Result: 存在 handlers/, routes/, models/, tasks/, middleware/ 目录
    Evidence: .omo/evidence/task-2-project-structure.txt
  ```

  **Commit**: YES
  - Message: `feat(backend): initialize Axum project skeleton`
  - Files: `backend/`
  - Pre-commit: `cargo check`

- [x] 3. 前端项目骨架搭建

  **What to do**:
  - 创建 Vue 3 + Vite 项目
  - 安装依赖：Naive UI, Pinia, Vue Router, Axios
  - 配置目录结构：`src/views/`, `src/components/`, `src/composables/`, `src/stores/`, `src/router/`
  - 创建基础布局组件（响应式）
  - 配置 Vite 代理到后端 API

  **Must NOT do**:
  - 不要实现具体页面（留给后续任务）
  - 不要添加测试配置（Task 16）

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 项目脚手架搭建，使用标准工具链
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4, 5)
  - **Blocks**: Tasks 10, 11, 12, 13, 14
  - **Blocked By**: Task 1

  **References**:

  **External References**:
  - Vue 3 + Vite 官方: https://vuejs.org/guide/quick-start.html
  - Naive UI 官方: https://www.naiveui.com/
  - Pinia 官方: https://pinia.vuejs.org/

  **Acceptance Criteria**:

  ```
  Scenario: 前端项目可启动
    Tool: Bash
    Preconditions: Node.js 工具链已安装
    Steps:
      1. cd frontend && npm install
      2. npm run dev &
      3. sleep 3 && curl http://localhost:5173
      4. kill %1
    Expected Result: npm run dev 成功，首页返回 HTML
    Evidence: .omo/evidence/task-3-frontend-skeleton.txt

  Scenario: 项目结构正确
    Tool: Bash
    Preconditions: 前端项目已创建
    Steps:
      1. ls -la frontend/src/
      2. find frontend/src -type d | sort
    Expected Result: 存在 views/, components/, composables/, stores/, router/ 目录
    Evidence: .omo/evidence/task-3-project-structure.txt
  ```

  **Commit**: YES
  - Message: `feat(frontend): initialize Vue 3 + Vite project`
  - Files: `frontend/`
  - Pre-commit: `npm run build`

- [x] 4. 数据库设计与迁移

  **What to do**:
  - 设计数据库 schema（posts, todo_items, subscribers, images）
  - 创建 SeaORM 迁移文件
  - 实现数据库连接配置
  - 创建基础 Model 结构体

  **Must NOT do**:
  - 不要实现 CRUD 逻辑（Task 6）
  - 不要添加种子数据

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 数据库设计和迁移，结构明确
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 5)
  - **Blocks**: Tasks 6, 7, 9
  - **Blocked By**: Task 1

  **References**:

  **External References**:
  - SeaORM 迁移文档: https://www.sea-ql.org/SeaORM/docs/migration/setting-up

  **Acceptance Criteria**:

  ```
  Scenario: 数据库迁移成功
    Tool: Bash
    Preconditions: PostgreSQL 已启动
    Steps:
      1. cd backend && cargo run -- migrate
      2. psql -d zyblog -c "\dt"
    Expected Result: 迁移成功，表已创建
    Evidence: .omo/evidence/task-4-migration.txt

  Scenario: Model 结构体可编译
    Tool: Bash
    Preconditions: SeaORM model 已生成
    Steps:
      1. cargo check
    Expected Result: 编译成功，无错误
    Evidence: .omo/evidence/task-4-models.txt
  ```

  **Commit**: YES
  - Message: `feat(db): design schema and create migrations`
  - Files: `backend/src/models/`, `backend/migrations/`
  - Pre-commit: `cargo check`

- [x] 5. Docker Compose 开发环境

  **What to do**:
  - 创建 `docker-compose.yml`（PostgreSQL, 后端, 前端）
  - 创建 `Dockerfile`（后端和前端）
  - 配置环境变量（`.env.example`）
  - 配置卷挂载（热重载）
  - 创建 `Makefile` 或 `justfile` 简化常用命令

  **Must NOT do**:
  - 不要配置生产环境（留给后续）
  - 不要添加 Nginx 反向代理（开发阶段不需要）

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Docker 配置，标准实践
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3, 4)
  - **Blocks**: None
  - **Blocked By**: Task 1

  **References**:

  **External References**:
  - Docker Compose 官方: https://docs.docker.com/compose/

  **Acceptance Criteria**:

  ```
  Scenario: Docker Compose 启动成功
    Tool: Bash
    Preconditions: Docker 已安装
    Steps:
      1. docker-compose up -d
      2. docker-compose ps
      3. curl http://localhost:8080/api/health
      4. curl http://localhost:5173
    Expected Result: 所有服务运行，API 和前端可访问
    Evidence: .omo/evidence/task-5-docker-compose.txt

  Scenario: 环境变量配置正确
    Tool: Bash
    Preconditions: .env.example 存在
    Steps:
      1. cat .env.example
      2. grep -c "DATABASE_URL\|ADMIN_KEY" .env.example
    Expected Result: 包含必要的环境变量配置
    Evidence: .omo/evidence/task-5-env-config.txt
  ```

  **Commit**: YES
  - Message: `feat(docker): setup development environment`
  - Files: `docker-compose.yml`, `Dockerfile`, `.env.example`, `Makefile`
  - Pre-commit: `docker-compose config`

- [x] 6. 文章 CRUD API

  **What to do**:
  - 实现文章创建、读取、更新、删除 API
  - `POST /api/v1/posts` - 创建文章（支持草稿/发布状态）
  - `GET /api/v1/posts` - 获取文章列表（分页）
  - `GET /api/v1/posts/:id` - 获取单篇文章
  - `PUT /api/v1/posts/:id` - 更新文章
  - `DELETE /api/v1/posts/:id` - 删除文章
  - 实现 #todo 标签解析（从 content 中提取）
  - 实现 slug 自动生成（基于标题）

  **Must NOT do**:
  - 不要实现认证中间件（Task 8）
  - 不要实现邮件通知（Task 9）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 核心业务逻辑，需要仔细设计
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 7, 8, 9)
  - **Blocks**: Task 15
  - **Blocked By**: Tasks 2, 4

  **References**:

  **Pattern References**:
  - Task 2 创建的项目结构：`backend/src/handlers/`, `backend/src/routes/`

  **External References**:
  - Axum 路由文档: https://docs.rs/axum/latest/axum/

  **Acceptance Criteria**:

  ```
  Scenario: 创建文章成功
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/posts \
           -H "Content-Type: application/json" \
           -d '{"title":"Test Post","content":"Hello #todo World","status":"draft"}'
    Expected Result: 返回 201，body 包含 id, title, slug, status
    Evidence: .omo/evidence/task-6-create-post.txt

  Scenario: 获取文章列表
    Tool: Bash (curl)
    Preconditions: 至少 1 篇文章已创建
    Steps:
      1. curl http://localhost:8080/api/v1/posts
    Expected Result: 返回 200，body 包含 items 数组和 total
    Evidence: .omo/evidence/task-6-list-posts.txt

  Scenario: #todo 标签解析
    Tool: Bash (curl)
    Preconditions: 文章包含 #todo 标签
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/posts \
           -H "Content-Type: application/json" \
           -d '{"title":"Todo Test","content":"Fix this #todo and that #todo","status":"draft"}'
      2. curl http://localhost:8080/api/v1/posts/1/todos
    Expected Result: 返回包含 2 个 todo_items 的列表
    Evidence: .omo/evidence/task-6-todo-parsing.txt

  Scenario: 创建文章失败（缺少标题）
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/posts \
           -H "Content-Type: application/json" \
           -d '{"content":"No title"}'
    Expected Result: 返回 400，body 包含错误信息
    Evidence: .omo/evidence/task-6-create-post-error.txt
  ```

  **Commit**: YES
  - Message: `feat(api): implement post CRUD with todo parsing`
  - Files: `backend/src/handlers/posts.rs`, `backend/src/routes/posts.rs`
  - Pre-commit: `cargo test`

- [x] 7. 图片上传 API

  **What to do**:
  - 实现图片上传端点 `POST /api/v1/images`
  - 支持格式：jpeg, png, gif, webp
  - 文件大小限制：5MB
  - 存储路径：`static/uploads/`
  - 生成缩略图（可选）
  - 返回图片 URL

  **Must NOT do**:
  - 不要实现图片删除（MVP 不需要）
  - 不要实现云存储（本地文件系统）

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 文件上传处理，需要安全考虑
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 8, 9)
  - **Blocks**: Task 15
  - **Blocked By**: Tasks 2, 4

  **References**:

  **External References**:
  - Axum Multipart: https://docs.rs/axum/latest/axum/extract/struct.Multipart.html

  **Acceptance Criteria**:

  ```
  Scenario: 图片上传成功
    Tool: Bash (curl)
    Preconditions: 后端服务运行，static/uploads/ 目录存在
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/images \
           -H "Authorization: Bearer $ADMIN_KEY" \
           -F "file=@test-image.jpg"
    Expected Result: 返回 201，body 包含 url 字段
    Evidence: .omo/evidence/task-7-upload-success.txt

  Scenario: 图片上传失败（文件过大）
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. dd if=/dev/zero of=large-image.jpg bs=1M count=10
      2. curl -X POST http://localhost:8080/api/v1/images \
           -H "Authorization: Bearer $ADMIN_KEY" \
           -F "file=@large-image.jpg"
    Expected Result: 返回 413，body 包含 "file too large" 错误
    Evidence: .omo/evidence/task-7-upload-too-large.txt

  Scenario: 图片上传失败（格式不支持）
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. echo "not an image" > test.txt
      2. curl -X POST http://localhost:8080/api/v1/images \
           -H "Authorization: Bearer $ADMIN_KEY" \
           -F "file=@test.txt"
    Expected Result: 返回 400，body 包含 "unsupported format" 错误
    Evidence: .omo/evidence/task-7-upload-invalid-format.txt
  ```

  **Commit**: YES
  - Message: `feat(api): implement image upload with validation`
  - Files: `backend/src/handlers/images.rs`, `backend/src/routes/images.rs`
  - Pre-commit: `cargo test`

- [x] 8. 管理员认证中间件

  **What to do**:
  - 实现 Bearer Token 认证中间件
  - 从环境变量读取 `ADMIN_KEY`
  - 保护所有写操作（POST, PUT, DELETE）
  - 读操作（GET）不需要认证
  - 返回 401 Unauthorized（无效 token）

  **Must NOT do**:
  - 不要实现 JWT（使用静态 Bearer Token）
  - 不要实现用户系统

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 中间件实现，需要理解 Axum 中间件模式
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7, 9)
  - **Blocks**: Task 15
  - **Blocked By**: Task 2

  **References**:

  **External References**:
  - Axum 中间件: https://docs.rs/axum/latest/axum/middleware/index.html

  **Acceptance Criteria**:

  ```
  Scenario: 认证成功（有效 token）
    Tool: Bash (curl)
    Preconditions: ADMIN_KEY=test-secret-key
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/posts \
           -H "Authorization: Bearer test-secret-key" \
           -H "Content-Type: application/json" \
           -d '{"title":"Auth Test","content":"Hello","status":"draft"}'
    Expected Result: 返回 201
    Evidence: .omo/evidence/task-8-auth-success.txt

  Scenario: 认证失败（无效 token）
    Tool: Bash (curl)
    Preconditions: ADMIN_KEY=test-secret-key
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/posts \
           -H "Authorization: Bearer wrong-key" \
           -H "Content-Type: application/json" \
           -d '{"title":"Auth Test","content":"Hello","status":"draft"}'
    Expected Result: 返回 401
    Evidence: .omo/evidence/task-8-auth-fail.txt

  Scenario: 认证失败（无 token）
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/posts \
           -H "Content-Type: application/json" \
           -d '{"title":"Auth Test","content":"Hello","status":"draft"}'
    Expected Result: 返回 401
    Evidence: .omo/evidence/task-8-auth-missing.txt

  Scenario: 读操作不需要认证
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. curl http://localhost:8080/api/v1/posts
    Expected Result: 返回 200（无需 Authorization header）
    Evidence: .omo/evidence/task-8-read-no-auth.txt
  ```

  **Commit**: YES
  - Message: `feat(auth): implement admin bearer token middleware`
  - Files: `backend/src/middleware/auth.rs`
  - Pre-commit: `cargo test`

- [x] 9. #todo 解析与邮件订阅服务

  **What to do**:
  - 实现 #todo 标签解析逻辑（从文章 content 提取）
  - 实现订阅者管理（`POST /api/v1/subscribers`）
  - 实现邮件发送服务（使用 SMTP 或事务邮件服务）
  - 文章更新时通知订阅者
  - 创建 `todo_items` 表关联

  **Must NOT do**:
  - 不要实现退订功能（MVP 简化）
  - 不要实现邮件模板（简单文本即可）

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: 复杂的业务逻辑，涉及异步任务和邮件发送
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7, 8)
  - **Blocks**: Task 15
  - **Blocked By**: Tasks 2, 4

  **References**:

  **External References**:
  - Lettre (Rust 邮件库): https://crates.io/crates/lettre

  **Acceptance Criteria**:

  ```
  Scenario: 订阅成功
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/subscribers \
           -H "Content-Type: application/json" \
           -d '{"email":"test@example.com"}'
    Expected Result: 返回 201
    Evidence: .omo/evidence/task-9-subscribe.txt

  Scenario: 重复订阅失败
    Tool: Bash (curl)
    Preconditions: test@example.com 已订阅
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/subscribers \
           -H "Content-Type: application/json" \
           -d '{"email":"test@example.com"}'
    Expected Result: 返回 409，body 包含 "already subscribed"
    Evidence: .omo/evidence/task-9-subscribe-duplicate.txt

  Scenario: 文章更新触发邮件通知
    Tool: Bash (curl)
    Preconditions: 订阅者存在，文章包含 #todo
    Steps:
      1. curl -X PUT http://localhost:8080/api/v1/posts/1 \
           -H "Authorization: Bearer $ADMIN_KEY" \
           -H "Content-Type: application/json" \
           -d '{"title":"Updated","content":"Fixed #todo item","status":"published"}'
      2. 检查邮件发送日志
    Expected Result: 更新成功，邮件已发送（或已排队）
    Evidence: .omo/evidence/task-9-update-notify.txt

  Scenario: #todo 标签解析正确
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/posts \
           -H "Authorization: Bearer $ADMIN_KEY" \
           -H "Content-Type: application/json" \
           -d '{"title":"Parse Test","content":"Task 1 #todo\nTask 2 #todo\nTask 3 #todo","status":"draft"}'
      2. curl http://localhost:8080/api/v1/posts/1/todos
    Expected Result: 返回 3 个 todo_items
    Evidence: .omo/evidence/task-9-parse-todos.txt
  ```

  **Commit**: YES
  - Message: `feat(todo): implement todo parsing and email subscription`
  - Files: `backend/src/handlers/subscribers.rs`, `backend/src/tasks/email.rs`
  - Pre-commit: `cargo test`

- [x] 10. 响应式布局框架

  **What to do**:
  - 使用 Naive UI 创建响应式布局
  - 移动端：单栏布局，底部导航
  - PC端：侧边栏 + 主内容区
  - 实现路由配置（首页、发布、文章详情）
  - 实现全局状态管理（Pinia store）

  **Must NOT do**:
  - 不要实现具体页面内容（留给后续任务）
  - 不要添加复杂动画

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: UI 布局和响应式设计
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: 响应式布局设计需要专业指导

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 11, 12, 13, 14)
  - **Blocks**: Tasks 11, 12, 13, 14
  - **Blocked By**: Task 3

  **References**:

  **External References**:
  - Naive UI 布局: https://www.naiveui.com/components/layout
  - Naive UI 响应式: https://www.naiveui.com/components/grid

  **Acceptance Criteria**:

  ```
  Scenario: 移动端布局正确
    Tool: Playwright
    Preconditions: 前端服务运行
    Steps:
      1. 设置 viewport 为 375x667（iPhone SE）
      2. 访问 http://localhost:5173
      3. 检查页面结构
    Expected Result: 单栏布局，底部导航可见，无横向滚动
    Evidence: .omo/evidence/task-10-mobile-layout.png

  Scenario: PC端布局正确
    Tool: Playwright
    Preconditions: 前端服务运行
    Steps:
      1. 设置 viewport 为 1920x1080
      2. 访问 http://localhost:5173
      3. 检查页面结构
    Expected Result: 侧边栏 + 主内容区布局
    Evidence: .omo/evidence/task-10-desktop-layout.png

  Scenario: 路由切换正常
    Tool: Playwright
    Preconditions: 前端服务运行
    Steps:
      1. 访问 http://localhost:5173
      2. 点击导航链接到 /publish
      3. 点击导航链接到 /posts
    Expected Result: 页面切换正常，URL 更新
    Evidence: .omo/evidence/task-10-routing.png
  ```

  **Commit**: YES
  - Message: `feat(ui): implement responsive layout framework`
  - Files: `frontend/src/components/Layout.vue`, `frontend/src/router/`
  - Pre-commit: `npm run build`

- [x] 11. 备忘录式发布界面

  **What to do**:
  - 创建极简发布界面（标题 + 正文输入框 + 发布按钮）
  - 支持基本格式（加粗、列表）- 使用简单工具栏
  - 支持图片插入（粘贴、上传按钮、拖拽）
  - 实现自动保存草稿（使用 @vueuse/core useDebounceFn）
  - 发布后可编辑

  **Must NOT do**:
  - 不要实现完整 Markdown 编辑器（使用简单富文本）
  - 不要实现复杂排版功能

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: 编辑器 UI 和交互设计
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: 编辑器交互需要专业指导

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 12, 13, 14)
  - **Blocks**: Task 15
  - **Blocked By**: Task 10

  **References**:

  **External References**:
  - @vueuse/core: https://vueuse.org/
  - Naive UI 输入组件: https://www.naiveui.com/components/input

  **Acceptance Criteria**:

  ```
  Scenario: 发布界面可访问
    Tool: Playwright
    Preconditions: 前端服务运行
    Steps:
      1. 访问 http://localhost:5173/publish
      2. 检查页面元素
    Expected Result: 存在标题输入框、正文编辑区、发布按钮
    Evidence: .omo/evidence/task-11-publish-page.png

  Scenario: 自动保存草稿
    Tool: Playwright
    Preconditions: 发布页面已打开
    Steps:
      1. 在标题输入框输入 "Test Title"
      2. 在正文编辑区输入 "Test Content"
      3. 等待 2 秒（防抖）
      4. 检查 localStorage
    Expected Result: localStorage 包含草稿数据
    Evidence: .omo/evidence/task-11-auto-save.png

  Scenario: 图片上传成功
    Tool: Playwright
    Preconditions: 发布页面已打开
    Steps:
      1. 点击图片上传按钮
      2. 选择测试图片
      3. 等待上传完成
    Expected Result: 图片插入到编辑区，显示预览
    Evidence: .omo/evidence/task-11-image-upload.png

  Scenario: 发布文章成功
    Tool: Playwright
    Preconditions: 发布页面已打开
    Steps:
      1. 输入标题和内容
      2. 点击发布按钮
      3. 等待跳转
    Expected Result: 跳转到文章详情页，文章已发布
    Evidence: .omo/evidence/task-11-publish-success.png
  ```

  **Commit**: YES
  - Message: `feat(ui): implement memo-style publishing interface`
  - Files: `frontend/src/views/Publish.vue`, `frontend/src/composables/useAutoSave.ts`
  - Pre-commit: `npm run build`

- [x] 12. 文章列表与详情页

  **What to do**:
  - 实现文章列表页（首页）
  - 支持分页加载
  - 显示文章标题、摘要、发布时间
  - 实现文章详情页
  - 显示 #todo 标签（高亮 + 订阅按钮）

  **Must NOT do**:
  - 不要实现搜索功能
  - 不要实现分类筛选

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: 列表和详情页 UI 设计
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: 页面布局需要专业指导

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 11, 13, 14)
  - **Blocks**: Task 15
  - **Blocked By**: Task 10

  **References**:

  **External References**:
  - Naive UI 列表: https://www.naiveui.com/components/list
  - Naive UI 分页: https://www.naiveui.com/components/pagination

  **Acceptance Criteria**:

  ```
  Scenario: 文章列表显示正确
    Tool: Playwright
    Preconditions: 至少 3 篇文章已创建
    Steps:
      1. 访问 http://localhost:5173
      2. 检查文章列表
    Expected Result: 显示 3 篇文章，包含标题和摘要
    Evidence: .omo/evidence/task-12-post-list.png

  Scenario: 分页功能正常
    Tool: Playwright
    Preconditions: 超过 10 篇文章
    Steps:
      1. 访问 http://localhost:5173
      2. 点击下一页
      3. 检查文章列表更新
    Expected Result: 列表更新，显示第 2 页文章
    Evidence: .omo/evidence/task-12-pagination.png

  Scenario: 文章详情页显示正确
    Tool: Playwright
    Preconditions: 文章已创建
    Steps:
      1. 访问 http://localhost:5173/posts/1
      2. 检查页面内容
    Expected Result: 显示完整文章内容
    Evidence: .omo/evidence/task-12-post-detail.png

  Scenario: #todo 标签高亮显示
    Tool: Playwright
    Preconditions: 文章包含 #todo 标签
    Steps:
      1. 访问包含 #todo 的文章详情页
      2. 检查 #todo 标签样式
    Expected Result: #todo 标签高亮显示，有订阅按钮
    Evidence: .omo/evidence/task-12-todo-highlight.png
  ```

  **Commit**: YES
  - Message: `feat(ui): implement post list and detail pages`
  - Files: `frontend/src/views/Home.vue`, `frontend/src/views/PostDetail.vue`
  - Pre-commit: `npm run build`

- [x] 13. Obsidian 导入界面

  **What to do**:
  - 创建文件上传界面
  - 支持 .md 文件上传
  - 解析 Markdown 内容（支持 frontmatter）
  - 预览导入内容
  - 确认导入后创建文章

  **Must NOT do**:
  - 不要支持 Obsidian 插件语法（dataview, templater）
  - 不要支持批量导入

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: 文件上传和预览 UI
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: 上传界面需要专业指导

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 11, 12, 14)
  - **Blocks**: Task 15
  - **Blocked By**: Task 10

  **References**:

  **External References**:
  - Naive UI 上传: https://www.naiveui.com/components/upload
  - front-matter (npm): https://www.npmjs.com/package/front-matter

  **Acceptance Criteria**:

  ```
  Scenario: 导入页面可访问
    Tool: Playwright
    Preconditions: 前端服务运行
    Steps:
      1. 访问 http://localhost:5173/import
      2. 检查页面元素
    Expected Result: 存在文件上传区域
    Evidence: .omo/evidence/task-13-import-page.png

  Scenario: 文件上传成功
    Tool: Playwright
    Preconditions: 导入页面已打开
    Steps:
      1. 创建测试 .md 文件（包含 frontmatter）
      2. 上传文件
      3. 检查预览
    Expected Result: 显示预览，标题和内容正确解析
    Evidence: .omo/evidence/task-13-upload-preview.png

  Scenario: 确认导入成功
    Tool: Playwright
    Preconditions: 文件已上传，预览显示
    Steps:
      1. 点击确认导入按钮
      2. 等待跳转
    Expected Result: 跳转到文章详情页，内容已导入
    Evidence: .omo/evidence/task-13-import-confirm.png

  Scenario: 导入失败（格式不支持）
    Tool: Playwright
    Preconditions: 导入页面已打开
    Steps:
      1. 上传 .txt 文件
      2. 检查错误提示
    Expected Result: 显示 "unsupported file format" 错误
    Evidence: .omo/evidence/task-13-import-error.png
  ```

  **Commit**: YES
  - Message: `feat(ui): implement Obsidian import interface`
  - Files: `frontend/src/views/Import.vue`
  - Pre-commit: `npm run build`

- [x] 14. #todo 订阅界面

  **What to do**:
  - 在文章详情页添加订阅组件
  - 显示 #todo 标签列表
  - 订阅表单（邮箱输入 + 提交按钮）
  - 订阅成功反馈
  - 显示当前订阅人数（可选）

  **Must NOT do**:
  - 不要实现退订功能
  - 不要实现用户登录

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: 订阅表单 UI
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: 表单设计需要专业指导

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 11, 12, 13)
  - **Blocks**: Task 15
  - **Blocked By**: Task 10

  **References**:

  **External References**:
  - Naive UI 表单: https://www.naiveui.com/components/form

  **Acceptance Criteria**:

  ```
  Scenario: 订阅组件显示正确
    Tool: Playwright
    Preconditions: 文章包含 #todo 标签
    Steps:
      1. 访问文章详情页
      2. 检查订阅组件
    Expected Result: 显示 #todo 列表和订阅表单
    Evidence: .omo/evidence/task-14-subscribe-component.png

  Scenario: 订阅成功
    Tool: Playwright
    Preconditions: 订阅组件已显示
    Steps:
      1. 输入邮箱 "test@example.com"
      2. 点击订阅按钮
      3. 检查成功提示
    Expected Result: 显示 "订阅成功" 提示
    Evidence: .omo/evidence/task-14-subscribe-success.png

  Scenario: 订阅失败（邮箱格式错误）
    Tool: Playwright
    Preconditions: 订阅组件已显示
    Steps:
      1. 输入邮箱 "invalid-email"
      2. 点击订阅按钮
      3. 检查错误提示
    Expected Result: 显示 "邮箱格式错误" 提示
    Evidence: .omo/evidence/task-14-subscribe-error.png

  Scenario: 订阅失败（已订阅）
    Tool: Playwright
    Preconditions: test@example.com 已订阅
    Steps:
      1. 输入邮箱 "test@example.com"
      2. 点击订阅按钮
      3. 检查错误提示
    Expected Result: 显示 "已订阅" 提示
    Evidence: .omo/evidence/task-14-subscribe-duplicate.png
  ```

  **Commit**: YES
  - Message: `feat(ui): implement todo subscription interface`
  - Files: `frontend/src/components/TodoSubscribe.vue`
  - Pre-commit: `npm run build`

- [x] 15. 前后端联调

  **What to do**:
  - 配置 Vite 代理到后端 API
  - 联调文章 CRUD 功能
  - 联调图片上传功能
  - 联调 #todo 订阅功能
  - 联调 Obsidian 导入功能
  - 修复接口对接问题

  **Must NOT do**:
  - 不要修改 API 设计（保持 Task 6-9 的实现）
  - 不要添加新功能

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 前后端集成，需要调试能力
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential
  - **Blocks**: Tasks 16, 17
  - **Blocked By**: Tasks 6, 7, 8, 9, 11, 12, 13, 14

  **References**:

  **Pattern References**:
  - Task 6-9 的 API 实现
  - Task 11-14 的前端组件

  **External References**:
  - Vite 代理配置: https://vitejs.dev/config/server-options.html#server-proxy

  **Acceptance Criteria**:

  ```
  Scenario: 文章发布流程完整
    Tool: Playwright
    Preconditions: 前后端服务运行
    Steps:
      1. 访问 http://localhost:5173/publish
      2. 输入标题和内容
      3. 点击发布
      4. 检查文章列表
    Expected Result: 文章发布成功，列表中显示新文章
    Evidence: .omo/evidence/task-15-publish-flow.png

  Scenario: 图片上传流程完整
    Tool: Playwright
    Preconditions: 发布页面已打开
    Steps:
      1. 上传图片
      2. 检查图片显示
      3. 发布文章
      4. 检查文章详情页图片
    Expected Result: 图片上传成功，文章中显示图片
    Evidence: .omo/evidence/task-15-image-flow.png

  Scenario: #todo 订阅流程完整
    Tool: Playwright
    Preconditions: 文章包含 #todo
    Steps:
      1. 访问文章详情页
      2. 输入邮箱订阅
      3. 更新文章
      4. 检查邮件发送日志
    Expected Result: 订阅成功，更新时邮件已发送
    Evidence: .omo/evidence/task-15-todo-flow.png

  Scenario: Obsidian 导入流程完整
    Tool: Playwright
    Preconditions: 准备测试 .md 文件
    Steps:
      1. 访问 http://localhost:5173/import
      2. 上传 .md 文件
      3. 预览内容
      4. 确认导入
      5. 检查文章列表
    Expected Result: 导入成功，文章列表中显示导入的文章
    Evidence: .omo/evidence/task-15-import-flow.png
  ```

  **Commit**: YES
  - Message: `feat(integration): frontend-backend integration`
  - Files: `frontend/vite.config.ts`, various fixes
  - Pre-commit: `npm run build && cargo test`

- [x] 16. 测试补充

  **What to do**:
  - 补充后端单元测试（handlers, models）
  - 补充前端组件测试（Vitest + Vue Test Utils）
  - 补充 API 集成测试
  - 配置测试覆盖率报告
  - 创建测试文档

  **Must NOT do**:
  - 不要实现 E2E 测试（Playwright，可选后续）
  - 不要追求 100% 覆盖率

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 测试编写，需要理解业务逻辑
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Task 17)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 15

  **References**:

  **Pattern References**:
  - Task 6-9 的 API 实现
  - Task 11-14 的前端组件

  **External References**:
  - Vitest 官方: https://vitest.dev/
  - Vue Test Utils: https://test-utils.vuejs.org/

  **Acceptance Criteria**:

  ```
  Scenario: 后端测试通过
    Tool: Bash
    Preconditions: 测试已编写
    Steps:
      1. cd backend && cargo test
      2. 检查测试结果
    Expected Result: 所有测试通过，覆盖率 > 60%
    Evidence: .omo/evidence/task-16-backend-tests.txt

  Scenario: 前端测试通过
    Tool: Bash
    Preconditions: 测试已编写
    Steps:
      1. cd frontend && vitest run
      2. 检查测试结果
    Expected Result: 所有测试通过，覆盖率 > 60%
    Evidence: .omo/evidence/task-16-frontend-tests.txt

  Scenario: 测试覆盖率报告生成
    Tool: Bash
    Preconditions: 覆盖率配置完成
    Steps:
      1. cd backend && cargo tarpaulin --out Html
      2. cd frontend && vitest run --coverage
    Expected Result: 生成 HTML 覆盖率报告
    Evidence: .omo/evidence/task-16-coverage.txt
  ```

  **Commit**: YES
  - Message: `test: add unit and integration tests`
  - Files: `backend/tests/`, `frontend/src/__tests__/`
  - Pre-commit: `cargo test && vitest run`

- [x] 17. 文档与导出功能

  **What to do**:
  - 创建 API 文档（使用 utoipa 或手写 OpenAPI）
  - 实现数据导出功能（JSON/CSV 格式）
  - 创建 README 使用文档
  - 创建开发文档

  **Must NOT do**:
  - 不要实现自动备份
  - 不要实现云存储

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 文档编写
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Task 16)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 15

  **References**:

  **External References**:
  - utoipa (Rust OpenAPI): https://crates.io/crates/utoipa

  **Acceptance Criteria**:

  ```
  Scenario: API 文档可访问
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. curl http://localhost:8080/swagger-ui/
    Expected Result: 返回 Swagger UI 页面
    Evidence: .omo/evidence/task-17-api-docs.txt

  Scenario: 数据导出成功
    Tool: Bash (curl)
    Preconditions: 后端服务运行，有文章数据
    Steps:
      1. curl -H "Authorization: Bearer $ADMIN_KEY" \
           http://localhost:8080/api/v1/export/posts \
           -o export.json
      2. cat export.json | jq '.[0].title'
    Expected Result: 导出 JSON 文件，包含所有文章
    Evidence: .omo/evidence/task-17-export.txt

  Scenario: README 文档完整
    Tool: Bash
    Preconditions: README.md 存在
    Steps:
      1. cat README.md | grep -c "## "
      2. cat README.md | grep -c "docker-compose"
    Expected Result: README 包含项目介绍、安装步骤、使用说明
    Evidence: .omo/evidence/task-17-readme.txt
  ```

  **Commit**: YES
  - Message: `docs: add API documentation and export feature`
  - Files: `docs/`, `backend/src/handlers/export.rs`, `README.md`
  - Pre-commit: N/A

---

## Final Verification Wave

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, curl endpoint, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .omo/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo test` + `vitest run` + linter. Review all changed files for: `unwrap()` in production code, empty catches, console.log in prod, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Tests [N pass/N fail] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high` (+ `playwright` skill)
  Start from clean state (`docker-compose down -v && docker-compose up`). Execute EVERY QA scenario from EVERY task — follow exact steps, capture evidence. Test cross-task integration. Test edge cases: empty state, invalid input, large files. Save to `.omo/evidence/final-qa/`.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Wave 1**: `feat(init): project scaffolding` - Git, backend, frontend, DB, Docker
- **Wave 2**: `feat(api): core backend APIs` - CRUD, auth, images, email
- **Wave 3**: `feat(ui): frontend components` - layout, editor, list, import, subscribe
- **Wave 4**: `feat(integration): full stack` - API integration, tests, docs
- **Final**: `chore: final verification and cleanup`

---

## Success Criteria

### Verification Commands
```bash
# 启动完整环境
docker-compose up -d

# 后端测试
cargo test

# 前端测试
vitest run

# API 测试
curl -X POST http://localhost:8080/api/v1/posts \
  -H "Authorization: Bearer $ADMIN_KEY" \
  -H "Content-Type: application/json" \
  -d '{"title":"Test","content":"Hello #todo","status":"draft"}'

# 前端访问
curl http://localhost:3000  # 应返回 HTML
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] All tests pass
- [ ] Docker Compose 环境可一键启动
- [ ] 移动端响应式布局正常
- [ ] #todo 标签高亮显示
- [ ] 邮件订阅功能正常
- [ ] Obsidian 文件导入正常
