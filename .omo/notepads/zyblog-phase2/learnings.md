## docs/api.md 中文化完成

- 保留了所有 API 端点路径和技术细节
- 翻译了所有描述性文本和注释
- 数据类型名称保持英文，添加了中文注释
- 格式和代码块完整保留
## docs/development.md 中文化

- 文件包含完整的开发指南和架构说明
- 翻译时保留了所有技术术语（如 Axum、SeaORM、PostgreSQL）
- 保留了所有代码块和链接格式
- 中文翻译准确，技术细节完整

## 代码注释中文化完成

- 后端 Rust 代码注释翻译完成（main.rs, handlers/posts.rs, handlers/subscribers.rs, handlers/images.rs, middleware/auth.rs, tasks/email.rs）
- 前端 Vue/TS 代码注释翻译完成（api.ts, app.ts, usePosts.ts, Layout.vue, PostDetail.vue, Home.vue, Import.vue, Publish.vue, TodoSubscribe.vue）
- 保留了所有技术细节和代码逻辑
- 仅翻译注释，未修改任何函数名、变量名或代码逻辑
- 通过 git diff 验证：所有变更均为注释翻译
- CSS 注释也已翻译（如 /* --- Header --- */ → /* --- 头部 --- */）
- 测试环境不可用（无 cargo/npx），但代码逻辑未变更

## PR 式互动前端界面实现

### 创建的文件
- `frontend/src/composables/usePullRequests.ts` - PR 相关 API 组合式函数
- `frontend/src/components/PullRequest.vue` - PR 交互组件

### 后端 API 端点（已存在）
- `GET /api/v1/posts/{id}/pulls` - 获取文章的 PR 列表
- `POST /api/v1/posts/{id}/pulls` - 创建 PR（body: { user_email, content }）
- `PUT /api/v1/pulls/{id}` - 更新 PR 状态（open/closed/merged）
- `POST /api/v1/pulls/{id}/comments` - 添加评论（body: { user_email, content }）

### 设计系统遵循
- 使用 Playfair Display + Source Sans 3 字体
- 遵循 4px 间距系统（--space-* 变量）
- 使用暖色调编辑风格（--color-accent: #C45D3E）
- 使用 Naive UI 组件（NButton, NInput, NSelect, NSpin）

### 组件功能
1. PR 列表显示：卡片式布局，显示状态、邮箱、内容、日期
2. 状态筛选：支持全部/开放/已关闭/已合并筛选
3. 创建 PR 表单：弹窗式，包含邮箱和内容字段
4. PR 详情视图：显示完整内容和状态操作按钮
5. 评论功能：支持添加评论，显示评论列表
6. 响应式设计：移动端适配

### 集成方式
- 在 PostDetail.vue 中导入并使用 PullRequest 组件
- 传入 postId 作为 prop
- 位于待办事项区域之后

## 环境配置完成（Task 5）

- .env 文件从 .env.example 复制并配置
- 管理员密钥设置为 zyblog_admin_2024_secure_key
- SMTP 邮件服务为可选配置，当前为空
- 创建了一键启动脚本 start.sh
- 脚本功能：检查 Docker、创建 .env、启动服务、等待就绪、显示状态
- 当前环境未安装 Docker，需要 sudo 权限安装
- 所有配置文件已就绪，安装 Docker 后可一键部署
- docker-compose.yml 已正确配置服务依赖和健康检查

## Phase 2 Task 6: PR式互动 DB Schema

### 现有代码发现
- 已存在 `pull.rs` (table: `pulls`) 和 `pull_comment.rs` (table: `pull_comments`) 模型
- 旧模型字段不同：`pulls` 有 title 和 updated_at，`pull_comments` 有 user_email
- 新建的 `pull_requests`/`pull_request_comments` 是不同表名，不冲突

### SeaORM 迁移模式
- 迁移命名：`m{date}_{seq}_{description}.rs`
- 外键使用 `ForeignKey::create()` 在 `create_table` 内链式调用
- `on_delete(ForeignKeyAction::Cascade)` 实现级联删除
- 索引需单独调用 `create_index`

### cargo 路径
- cargo 在 `$HOME/.cargo/bin/cargo`，需设置 PATH

## 视频上传 API 实现（Task 9）

- 创建了 handlers/videos.rs，遵循 images.rs 的模式
- 支持格式：mp4, webm, ogg（MIME 类型验证）
- 文件大小限制：100MB（DefaultBodyLimit 设为 105MB 以容纳 multipart 开销）
- 存储路径：static/videos/
- 路由 POST /api/v1/videos 需要认证（放在 protected_routes 中）
- 修复了 migrations/mod.rs 中的多余花括号（预存 bug）
- cargo test --lib 通过（6 个测试全部成功）
- 集成测试失败是预存问题（Config 缺少 backup 相关字段）

## Task 8: Database Backup Feature

### Files Created
- `backend/src/handlers/backup.rs` - API handlers (create/list/restore)
- `backend/src/routes/backup.rs` - Route definitions
- `backend/src/tasks/backup.rs` - Core backup logic + scheduled task
- `backend/src/models/pull.rs` - Missing model (pre-existing issue)
- `backend/src/models/pull_comment.rs` - Missing model (pre-existing issue)

### Files Modified
- `backend/src/config.rs` - Added backup_dir, backup_interval_hours, backup_retention_count
- `backend/src/main.rs` - Registered backup routes + scheduled backup task
- `backend/src/handlers/mod.rs` - Added backup module
- `backend/src/routes/mod.rs` - Added backup module
- `backend/src/tasks/mod.rs` - Added backup module
- `backend/src/models/mod.rs` - Added pull, pull_comment modules
- `backend/Cargo.toml` - Added flate2, tempfile dependencies
- `backend/tests/api_integration_test.rs` - Added backup config fields

### Pre-existing Issues Fixed
- `models/mod.rs` declared `pull` and `pull_comment` modules that didn't exist
- `migrations/mod.rs` had extra closing brace at line 27

### Backup Implementation
- Uses `pg_dump` for creating backups (compressed with gzip via flate2)
- Uses `psql` for restoring from backups
- Backups stored as `.sql.gz` files with timestamps
- Automatic cleanup of old backups based on retention count
- Scheduled task runs on configurable interval (default 24h)

### Environment Variables
- `BACKUP_DIR` - Backup directory path (default: ./backups)
- `BACKUP_INTERVAL_HOURS` - Backup interval in hours (default: 24)
- `BACKUP_RETENTION_COUNT` - Max backup files to keep (default: 10)

### API Endpoints
- POST `/api/v1/backup` - Create backup (requires auth)
- GET `/api/v1/backup/list` - List backups (requires auth)
- POST `/api/v1/backup/restore` - Restore from backup (requires auth)

## Task 7: PR式互动 API

### Key Learnings
- Task 6 (DB design) already created models (`pull_request`, `pull_request_comment`) and migrations. Always check existing models before creating new ones to avoid duplicates.
- The existing `pull_request` model uses table `pull_requests` with columns: id, post_id, user_email, content, status, created_at (no title, no updated_at).
- The existing `pull_request_comment` model uses table `pull_request_comments` with columns: id, pull_request_id, content, created_at (no user_email).
- Route nesting in Axum: `nest("/api/v1/posts/{id}/pulls", ...)` for nested routes under a path parameter.
- The `CommentResponse` stores `user_email` from the request body since the DB model doesn't have it.
- Status validation: only "open", "closed", "merged" are valid PR states.
- All 45 tests pass after implementation.

## Task 11: Backup Management UI

### Backup API Endpoints (backend)
- `POST /api/v1/backup` → creates backup, returns `{message, filename, size_bytes}`
- `GET /api/v1/backup/list` → lists backups, returns `{backups: [{filename, size_bytes, created_at}], total}`
- `POST /api/v1/backup/restore` → restores from backup, body `{filename}`, returns `{message, filename}`

### Navigation Pattern
- Menu items defined in `Layout.vue` as computed `menuOptions` array with `{label, key, icon}`
- Mobile nav uses separate `navIconMap` Record<string, string>
- Icon uses `renderIcon()` helper that returns `() => h('span', {...}, icon)` render function
- Unicode characters used as icons (⌂, ✎, ☰, ↓, ⤓)

### Build Environment
- Node.js not pre-installed; need nvm to set up
- `@types/node` v25.x incompatible with vue-tsc 5.6.3; use `@types/node@20`
- `vue-tsc -b` has pre-existing errors in PullRequest.vue, test files, PostDetail.vue
- `vite build` works independently and produces correct output
- tsconfig.node.json originally missing @types/node; added via npm install

### Design System Tokens Used
- Colors: `--color-bg-elevated`, `--color-text-primary/secondary/tertiary`, `--color-accent`, `--color-border-light`, `--color-bg-sunken`
- Spacing: `--space-1` through `--space-16`
- Typography: `--font-display`, `--font-body`, `--font-mono`, `--text-xs` through `--text-3xl`
- Shadows: `--shadow-sm`, `--shadow-md`, `--shadow-lg`
- Radii: `--radius-sm`, `--radius-md`, `--radius-lg`
- Transitions: `--transition-fast`, `--transition-base`

### Component Pattern (from Import.vue)
- Auth dialog: `showAuthDialog` ref + overlay + `handleAuthSubmit()`
- Error handling: check `err.response?.status === 401` for auth errors
- File size formatting helper for human-readable display
- NPopconfirm for destructive actions (restore)

## 前端空白页面修复

### 根本原因
- `App.vue` 的 `themeOverrides` 使用 CSS 变量（如 `var(--color-accent)`）作为 Naive UI 的颜色值
- Naive UI 内部使用 `seemly` 库的 `rgba()` 函数处理颜色，该函数无法解析 CSS 变量
- 错误信息：`[seemly/rgba]: Invalid color value var(--color-bg)`
- 该错误导致 Vue 应用挂载时崩溃，页面显示空白

### 修复方案
- 将 `themeOverrides` 中的 CSS 变量替换为实际颜色值（十六进制）
- `var(--color-accent)` → `#C45D3E`
- `var(--color-accent-hover)` → `#A94E34`
- `var(--color-accent-light)` → `#F5E6E0`
- `var(--color-bg)` → `#FAFAF7`
- `var(--color-bg-elevated)` → `#FFFFFF`
- `var(--font-body)` → `'Source Sans 3', 'Segoe UI', system-ui, -apple-system, sans-serif`

### 关键教训
- Naive UI 的 `GlobalThemeOverrides` 接受实际颜色值（hex/rgb），不支持 CSS 变量
- 使用 `vitest` + `happy-dom` 可以在无浏览器环境下诊断前端运行时错误
- 测试方法：`mount(App, { global: { plugins: [pinia, router] } })` 捕获异常

## 文档更新（Task 15）

### 更新内容
- README.md：在技术栈后添加"新功能"部分，涵盖 PR 式互动、自动备份、视频上传三大功能
- docs/api.md：添加 PR、备份、视频上传的完整 API 端点文档和数据类型定义
- docs/development.md：更新后端架构目录结构，添加数据库表说明，新增"新功能开发指南"章节

### README.md 更新要点
- 新功能部分放在"技术栈"和"快速开始"之间，采用场景描述+功能特点+限制的结构
- 三个功能各自独立成小节，便于读者快速定位
- API 概览部分已包含 PR、备份、视频上传的端点表格（原有内容）

### docs/api.md 更新要点
- 在"导出"和"静态文件"之间插入 PR、备份、视频上传三个 API 章节
- 每个端点包含方法、路径、请求体、响应说明
- 数据类型部分新增 PullResponse、PullListResponse、CommentResponse、BackupResponse、BackupListResponse、RestoreResponse

### docs/development.md 更新要点
- 后端目录结构新增 handlers/pulls.rs、handlers/backup.rs、handlers/videos.rs
- 路由目录同步新增对应模块
- 数据库表新增 pull_requests、pull_request_comments
- tasks 目录新增 backup.rs
- 新增"新功能开发指南"章节，覆盖 PR 互动、数据备份、视频上传的实现细节

### 教训
- notepad 文件是 append-only 的，不能用 Write 工具覆盖，只能用 Edit 追加
- 文档更新前应先通读现有内容，避免重复或冲突
- 中文文档中技术术语（如 API、PR、PostgreSQL）保持英文原样

## Phase 2 Task 13: 端到端验证

### 验证结果
- 数据库: PostgreSQL 18 集群关闭 (down)，需要 sudo 启动
- 后端 API: 健康检查通过 (200 OK)，数据操作失败 (数据库不可用)
- 前端页面: Vue 3 SPA 正常运行 (200 OK)
- 新功能: 无法测试 (数据库不可用)

### 后端降级模式
- 后端支持无数据库启动模式: "Database not available, running without DB"
- 健康检查端点 /api/health 不依赖数据库，返回 {"status":"ok"}
- 数据端点 (如 /api/v1/posts) 返回 {"error":"Internal server error"}

### PostgreSQL 启动
- 集群版本: PostgreSQL 18
- 数据目录: /var/lib/postgresql/18/main
- 启动命令: sudo pg_ctlcluster 18 main start
- 需要 sudo 权限 (用户 zy 在 sudo 组但需要密码)

### tmux 会话
- backend: 后端服务 (cargo run)
- frontend: 前端服务 (vite dev)
- setup: 初始设置 (有 sudo 提示等待密码)
- zyblog: 旧会话

### 证据文件
- .omo/evidence/phase2-task-13-db-connection.txt
- .omo/evidence/phase2-task-13-backend-api.txt
- .omo/evidence/phase2-task-13-frontend-page.txt
- .omo/evidence/phase2-task-13-new-features.txt

## Phase 2 Task 14: 新功能单元测试

### 新增测试文件

**后端 (Rust):**
- `backend/tests/pulls_test.rs` - 10 tests (PR 请求类型反序列化 + 路由测试)
- `backend/tests/backup_test.rs` - 8 tests (备份请求反序列化 + 路径遍历防护 + 路由测试)
- `backend/tests/videos_test.rs` - 6 tests (视频上传 multipart 测试: 缺失字段/不支持格式/有效mp4/webm)

**前端 (Vitest):**
- `frontend/src/__tests__/composables/usePullRequests.test.ts` - 20 tests (composable 状态/API调用/错误处理/工具函数)
- `frontend/src/__tests__/components/VideoPlayer.test.ts` - 16 tests (组件渲染/MIME推断/props/错误状态)
- `frontend/src/__tests__/views/Backup.test.ts` - 6 tests (页面渲染/标题/认证状态)

### 测试结果
- 后端新增 24 tests 全部通过
- 前端 8 个测试文件 79 个测试全部通过
- 预存问题: `config_test.rs::config_from_env_invalid_port_falls_back` 有并行测试竞态条件（非本次修改）

### 后端测试模式
- 路由测试使用 `tower::util::ServiceExt::oneshot` 模式
- 测试 AppState 使用 `db: None` 模拟无数据库场景
- multipart 测试使用手动构造 boundary + Content-Disposition 头
- 视频路由 `video_routes()` 不需要 AppState（无状态路由）

### 前端测试模式
- Vitest + happy-dom 环境
- Mock `../../lib/api` 的 get/post/put 方法
- 使用 `@vue/test-utils` 的 `mount()` 测试组件
- 使用 `@pinia/testing` 的 `createTestingPinia()` 测试 store 依赖
- naive-ui 组件通过 `vi.mock` 替换为简单 template

### 关键发现
- backup handler 的 `restore_backup` 有路径遍历防护（检查 `..`、`/`、`\`）
- video handler 支持 mp4/webm/ogg 三种格式，100MB 限制
- 使用 `uuid::Uuid::new_v4()` 生成唯一文件名
- pull_request_comment 模型没有 user_email 字段，handler 从请求体获取

## F4 Scope Fidelity Check Results

- 13/15 tasks fully compliant with spec
- Task 5 violated "不要修改现有代码" constraint - included Task 8's backup config changes
- Cross-task contamination: dead model files (pull.rs, pull_comment.rs) from abandoned first attempt
- Migration default status "pending" vs handler "open" - minor inconsistency
- All Must Have items implemented, all Must NOT Have items absent
- Unaccounted files: run.txt, test artifacts, evidence files (all low impact)
- VERDICT: CONDITIONAL APPROVE
