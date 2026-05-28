# ZYBlog 工程文档

本文档是 ZYBlog 项目的完整工程参考。读完之后，你就能理解整个系统的架构、技术选型、数据模型、API 设计、前后端实现细节，以及如何扩展。

---

## 1. 系统架构总览

### 1.1 架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                        浏览器 (Client)                          │
│  Vue 3 SPA + Naive UI + Pinia + Vue Router                      │
└──────────────────────────┬──────────────────────────────────────┘
                           │ HTTP (REST API)
                           │ /api/v1/*
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                    后端服务器 (Axum)                              │
│                                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │ 中间件层  │→│ 路由层    │→│ Handler层 │→│ Model层   │        │
│  │ Auth     │  │ routes/  │  │ handlers/│  │ models/  │        │
│  └──────────┘  └──────────┘  └──────────┘  └────┬─────┘        │
│                                                  │               │
│  ┌──────────┐  ┌──────────┐                      │               │
│  │ Task层   │  │ Storage层│                      │               │
│  │ email    │  │ local    │                      │               │
│  │ backup   │  └──────────┘                      │               │
│  └──────────┘                                    │               │
└──────────────────────────────────────────────────┼───────────────┘
                                                   │
                                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PostgreSQL 15                                 │
│  posts | users | tags | comments | pull_requests | ...          │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 前后端分离

前端是独立的 Vue 3 SPA，通过 REST API 与后端通信。开发模式下，Vite 将 `/api` 请求代理到 `http://localhost:8080`。生产环境通过 Docker Compose 编排，前端和后端各自独立运行。

### 1.3 数据流

```
用户操作 → Vue 组件 → Composable (API 调用) → Axios → 后端 API
                                                         ↓
                                              Auth 中间件验证
                                                         ↓
                                              Handler 处理业务逻辑
                                                         ↓
                                              SeaORM 查询/写入 PostgreSQL
                                                         ↓
                                              JSON 响应返回前端
                                                         ↓
                                              Pinia 状态更新 → UI 渲染
```

---

## 2. 技术栈详解

### 2.1 后端

| 技术 | 版本 | 用途 |
|------|------|------|
| Axum | 0.7 | 异步 HTTP 框架，基于 tokio |
| SeaORM | 1.x | Rust ORM，支持 PostgreSQL |
| PostgreSQL | 15 | 主数据库 |
| serde + serde_json | - | JSON 序列化/反序列化 |
| thiserror | - | 错误类型派生 |
| utoipa | - | OpenAPI 3.0 自动生成 |
| lettre | - | SMTP 邮件发送 |
| bcrypt | - | 密码哈希 |
| jsonwebtoken | - | JWT 签发和验证 |
| chrono | - | 时间处理 |
| tracing | - | 结构化日志 |
| tokio | - | 异步运行时 |
| tower-http | - | HTTP 中间件（静态文件、body 限制） |
| dotenvy | - | 环境变量加载 |

### 2.2 前端

| 技术 | 版本 | 用途 |
|------|------|------|
| Vue | 3.5 | UI 框架，Composition API |
| Vite | 5 | 构建工具和开发服务器 |
| Pinia | 3 | 状态管理 |
| Vue Router | 5 | 路由管理 |
| Naive UI | - | UI 组件库 |
| Axios | - | HTTP 客户端 |
| TypeScript | 5+ | 类型安全（严格模式） |
| Vitest | - | 单元测试框架 |
| happy-dom | - | 测试用 DOM 环境 |
| @vue/test-utils | - | Vue 组件测试工具 |

### 2.3 认证方案

系统支持两种认证方式并存：

- **JWT 认证**：用户通过注册/登录获取 access_token（15 分钟）和 refresh_token（7 天）。refresh_token 一次性使用。
- **ADMIN_KEY 认证**：环境变量中的静态令牌，适合脚本和 CI/CD。写操作需要携带 `Authorization: Bearer <ADMIN_KEY>`。

### 2.4 部署

Docker Compose 编排三个服务：PostgreSQL、后端、前端。Makefile 提供快捷命令。

---

## 3. 目录结构说明

### 3.1 后端目录结构

```
backend/
├── src/
│   ├── main.rs              # 入口：路由组装、OpenAPI、备份调度、管理员初始化
│   ├── lib.rs               # 模块导出（供集成测试使用）
│   ├── config.rs            # Config 结构体，从环境变量加载
│   ├── state.rs             # AppState { db: Option<DatabaseConnection>, config: Config }
│   ├── error.rs             # AppError 枚举，统一错误处理
│   ├── utils/               # 工具模块
│   │   ├── mod.rs           # 导出 diff_engine 和 fragment_mapper
│   │   ├── diff_engine.rs   # 文本 diff 生成和应用
│   │   └── fragment_mapper.rs # PR 片段验证和应用
│   ├── storage/             # 文件存储抽象
│   │   ├── mod.rs
│   │   └── local.rs         # 本地文件系统存储
│   ├── handlers/            # 请求处理器（业务逻辑核心）
│   │   ├── mod.rs
│   │   ├── auth.rs          # 注册、登录、JWT 刷新、GitHub OAuth
│   │   ├── posts.rs         # 文章 CRUD、搜索、#todo 提取
│   │   ├── pulls.rs         # PR 创建、更新、合并、评论
│   │   ├── revisions.rs     # 文章版本快照、回滚、diff
│   │   ├── comments.rs      # 评论提交、审核
│   │   ├── tags.rs          # 标签 CRUD、文章标签关联
│   │   ├── todos.rs         # TODO 列表、订阅、完成通知
│   │   ├── subscribers.rs   # 订阅者管理
│   │   ├── media.rs         # 媒体文件上传、列表、删除
│   │   ├── images.rs        # 图片上传
│   │   ├── videos.rs        # 视频上传
│   │   ├── backup.rs        # 备份创建、列表、恢复
│   │   └── export.rs        # 文章导出（JSON/CSV）
│   ├── models/              # SeaORM 实体定义
│   │   ├── mod.rs
│   │   ├── post.rs          # posts 表
│   │   ├── user.rs          # users 表
│   │   ├── tag.rs           # tags 表
│   │   ├── post_tag.rs      # post_tags 关联表
│   │   ├── comment.rs       # comments 表
│   │   ├── pull_request.rs  # pull_requests 表
│   │   ├── pull_request_comment.rs # pull_request_comments 表
│   │   ├── pr_comment.rs    # pr_comments 表
│   │   ├── post_revision.rs # post_revisions 表
│   │   ├── todo_item.rs     # todo_items 表
│   │   ├── todo_subscription.rs # todo_subscriptions 表
│   │   ├── subscriber.rs    # subscribers 表
│   │   ├── image.rs         # images 表
│   │   ├── media.rs         # media 表
│   │   └── refresh_token.rs # refresh_tokens 表
│   ├── routes/              # 路由分组定义
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── posts.rs
│   │   ├── pulls.rs
│   │   ├── revisions.rs
│   │   ├── comments.rs
│   │   ├── tags.rs
│   │   ├── todos.rs
│   │   ├── subscribers.rs
│   │   ├── media.rs
│   │   ├── images.rs
│   │   ├── videos.rs
│   │   ├── backup.rs
│   │   └── export.rs
│   ├── middleware/           # 中间件
│   │   ├── auth.rs          # Bearer token 认证（ADMIN_KEY）
│   │   └── jwt.rs           # JWT 认证和 Claims 解析
│   ├── migrations/          # SeaORM 迁移文件（20 个）
│   └── tasks/               # 后台任务
│       ├── mod.rs
│       ├── email.rs         # SMTP 邮件通知
│       └── backup.rs        # 定时备份（pg_dump）
├── tests/                   # 集成测试
├── static/                  # 静态文件（上传的图片/视频）
└── Cargo.toml
```

### 3.2 前端目录结构

```
frontend/
├── src/
│   ├── main.ts              # 应用入口：createApp + Pinia + Router
│   ├── App.vue              # 根组件
│   ├── style.css            # 全局样式
│   ├── router/
│   │   └── index.ts         # 路由定义（14 条路由，全部懒加载）
│   ├── stores/
│   │   └── app.ts           # Pinia 全局状态（侧边栏、用户信息）
│   ├── lib/
│   │   └── api.ts           # Axios 实例，自动附加 JWT，401 自动刷新
│   ├── composables/         # 组合式函数（API 调用层）
│   │   ├── useAuth.ts       # 认证：登录、注册、JWT 刷新、GitHub OAuth
│   │   ├── usePosts.ts      # 文章 CRUD
│   │   ├── usePullRequests.ts # PR 交互
│   │   ├── useRevisions.ts  # 版本管理
│   │   ├── useComments.ts   # 评论管理
│   │   ├── useTags.ts       # 标签管理
│   │   ├── useTodos.ts      # TODO 管理
│   │   ├── useMedia.ts      # 媒体文件管理
│   │   ├── useSearch.ts     # 文章搜索
│   │   └── useResponsive.ts # 响应式布局辅助
│   ├── components/          # 可复用 UI 组件
│   │   ├── Layout.vue       # 应用外壳（侧边栏 + 内容区）
│   │   ├── ArticleCard.vue  # 文章卡片
│   │   ├── HeroSection.vue  # 首页英雄区
│   │   ├── MarkdownRenderer.vue # Markdown 渲染
│   │   ├── PullRequest.vue  # PR 交互组件
│   │   ├── VideoPlayer.vue  # 视频播放器
│   │   └── TodoSubscribe.vue # TODO 订阅组件
│   ├── views/               # 页面组件（路由目标）
│   │   ├── Home.vue         # 首页/文章列表 (/)
│   │   ├── PostDetail.vue   # 文章详情 (/posts/:id)
│   │   ├── Publish.vue      # 创建/编辑文章 (/publish, /publish/:id)
│   │   ├── Drafts.vue       # 草稿管理 (/drafts)
│   │   ├── Import.vue       # 文章导入 (/import)
│   │   ├── Backup.vue       # 备份管理 (/backup)
│   │   ├── Comments.vue     # 评论审核 (/comments)
│   │   ├── Media.vue        # 媒体管理 (/media)
│   │   ├── Todos.vue        # TODO 管理 (/todos)
│   │   ├── Tags.vue         # 标签管理 (/tags)
│   │   ├── Login.vue        # 登录页 (/login)
│   │   └── GitHubCallback.vue # GitHub OAuth 回调 (/github/callback)
│   └── __tests__/           # 测试文件
│       ├── views/
│       ├── components/
│       └── composables/
├── index.html
├── vite.config.ts
├── tsconfig.json
└── package.json
```

### 3.3 关键文件说明

| 文件 | 职责 |
|------|------|
| `backend/src/main.rs` | 应用入口。组装路由、启动备份调度器、初始化管理员账户、配置 OpenAPI |
| `backend/src/state.rs` | 共享状态定义。`AppState` 包含可选的数据库连接和配置 |
| `backend/src/error.rs` | 统一错误类型。`AppError` 枚举映射到 HTTP 状态码和 JSON 错误响应 |
| `backend/src/middleware/auth.rs` | 认证中间件。GET 公开，POST/PUT/DELETE 需要 ADMIN_KEY |
| `backend/src/utils/fragment_mapper.rs` | PR 片段引擎。验证片段不重叠、不越界，将片段应用到文章内容 |
| `backend/src/utils/diff_engine.rs` | 文本 diff 引擎。生成 unified diff 格式，用于版本比较 |
| `frontend/src/lib/api.ts` | Axios 实例。自动附加 JWT，401 时自动刷新 token 并重试 |
| `frontend/src/composables/useAuth.ts` | 认证 composable。管理用户状态、登录/注册/token 刷新 |
| `frontend/src/router/index.ts` | 路由定义。导航守卫保护管理员路由 |

---

## 4. 数据库设计

### 4.1 ER 图

```
┌──────────┐       ┌──────────────┐       ┌──────────┐
│  users   │       │  post_tags   │       │   tags   │
│──────────│       │──────────────│       │──────────│
│ id (PK)  │◄──┐   │ post_id (FK) │   ┌──►│ id (PK)  │
│ email    │   │   │ tag_id  (FK) │───┘   │ name     │
│ name     │   │   └──────────────┘       │ slug     │
│ role     │   │                          └──────────┘
│ github_id│   │
│ password │   │   ┌──────────────┐       ┌──────────────┐
└──────────┘   │   │    posts     │       │post_revisions│
               │   │──────────────│       │──────────────│
┌──────────┐   │   │ id (PK)      │◄──────│ post_id (FK) │
│ refresh_ │   ├───│ author_id(FK)│       │ version      │
│ tokens   │   │   │ title        │       │ title        │
│──────────│   │   │ slug         │       │ content      │
│ id (PK)  │   │   │ content      │       │ created_by   │
│ user_id  │───┘   │ published    │       └──────────────┘
│ token    │       │ current_ver  │
│ expires  │       └──────┬───────┘
└──────────┘              │
                          │
    ┌─────────────────────┼─────────────────────┐
    │                     │                     │
    ▼                     ▼                     ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ todo_items   │  │  comments    │  │pull_requests │
│──────────────│  │──────────────│  │──────────────│
│ id (PK)      │  │ id (PK)      │  │ id (PK)      │
│ post_id (FK) │  │ post_id (FK) │  │ post_id (FK) │
│ title        │  │ author_name  │  │ user_email   │
│ completed    │  │ content      │  │ status       │
└──────────────┘  │ approved     │  │ fragments    │
                  └──────────────┘  └──────┬───────┘
                                           │
┌──────────────┐  ┌──────────────┐         ▼
│  media       │  │  images      │  ┌──────────────┐
│──────────────│  │──────────────│  │pr_comments   │
│ id (PK)      │  │ id (PK)      │  │──────────────│
│ filename     │  │ filename     │  │ id (PK)      │
│ mime_type    │  │ url          │  │ pr_id (FK)   │
│ size_bytes   │  └──────────────┘  │ content      │
│ uploaded_by  │                    │ fragment_idx │
└──────────────┘                    └──────────────┘

┌──────────────┐  ┌──────────────────┐
│ subscribers  │  │todo_subscriptions│
│──────────────│  │──────────────────│
│ id (PK)      │  │ id (PK)          │
│ email        │  │ todo_item_id(FK) │
│ confirmed    │  │ email / user_id  │
└──────────────┘  └──────────────────┘
```

### 4.2 表结构详解

#### posts（文章）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `title` | VARCHAR | 文章标题 |
| `slug` | VARCHAR | URL 友好标识符，从标题自动生成 |
| `content` | TEXT | 文章完整内容 |
| `excerpt` | VARCHAR? | 摘要 |
| `cover_image` | VARCHAR? | 封面图 URL |
| `published` | BOOLEAN | 是否已发布 |
| `author_id` | INT? | 作者 ID，外键关联 users |
| `current_version` | INT | 当前版本号 |
| `created_at` | TIMESTAMPTZ | 创建时间 |
| `updated_at` | TIMESTAMPTZ | 更新时间 |

#### users（用户）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `email` | VARCHAR | 邮箱，唯一 |
| `password_hash` | VARCHAR? | bcrypt 密码哈希 |
| `name` | VARCHAR | 用户名 |
| `avatar_url` | VARCHAR? | 头像 URL |
| `role` | VARCHAR | 角色：admin / visitor / contributor |
| `github_id` | VARCHAR? | GitHub 用户 ID |
| `created_at` | TIMESTAMPTZ | 创建时间 |
| `updated_at` | TIMESTAMPTZ | 更新时间 |

#### tags（标签）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `name` | VARCHAR | 标签名，唯一 |
| `slug` | VARCHAR | URL 友好标识符 |
| `created_at` | TIMESTAMPTZ | 创建时间 |

#### post_tags（文章标签关联）

| 列名 | 类型 | 说明 |
|------|------|------|
| `post_id` | INT FK | 文章 ID |
| `tag_id` | INT FK | 标签 ID |

联合主键 `(post_id, tag_id)`。

#### comments（评论）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `post_id` | INT FK | 文章 ID |
| `author_name` | VARCHAR | 评论者昵称 |
| `author_email` | VARCHAR? | 评论者邮箱 |
| `content` | TEXT | 评论内容 |
| `approved` | BOOLEAN | 是否已审核 |
| `referenced_content` | TEXT? | 引用的文章片段 |
| `created_at` | TIMESTAMPTZ | 创建时间 |
| `updated_at` | TIMESTAMPTZ | 更新时间 |

#### pull_requests（PR）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `post_id` | INT FK | 文章 ID |
| `user_email` | VARCHAR | 提交者邮箱 |
| `content` | TEXT | PR 摘要 |
| `status` | VARCHAR | 状态：open / closed / merged |
| `fragments` | JSONB? | 片段列表（精确修改位置） |
| `user_id` | INT? | 提交者用户 ID |
| `message` | VARCHAR? | PR 描述 |
| `created_at` | TIMESTAMPTZ | 创建时间 |

#### pr_comments（PR 评论）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `pull_request_id` | INT FK | PR ID |
| `fragment_index` | INT? | 片段索引 |
| `line` | INT? | 行号 |
| `content` | TEXT | 评论内容 |
| `user_email` | VARCHAR? | 评论者邮箱 |
| `created_at` | TIMESTAMPTZ | 创建时间 |

#### post_revisions（文章修订版本）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `post_id` | INT FK | 文章 ID |
| `title` | VARCHAR? | 快照时的标题 |
| `content` | TEXT? | 快照时的内容 |
| `excerpt` | VARCHAR? | 快照时的摘要 |
| `cover_image` | VARCHAR? | 快照时的封面图 |
| `version` | INT | 版本号 |
| `created_by` | INT? | 创建者用户 ID |
| `created_at` | TIMESTAMPTZ | 创建时间 |

#### todo_items（待办事项）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `post_id` | INT FK | 文章 ID |
| `title` | VARCHAR | TODO 标题 |
| `description` | VARCHAR? | 描述 |
| `completed` | BOOLEAN | 是否已完成 |
| `created_at` | TIMESTAMPTZ | 创建时间 |
| `updated_at` | TIMESTAMPTZ | 更新时间 |

#### todo_subscriptions（TODO 订阅）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `todo_item_id` | INT FK | TODO ID |
| `email` | VARCHAR? | 订阅者邮箱 |
| `user_id` | INT? | 订阅者用户 ID |
| `created_at` | TIMESTAMPTZ | 创建时间 |

#### subscribers（邮件订阅者）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `email` | VARCHAR | 邮箱，唯一 |
| `name` | VARCHAR? | 名称 |
| `confirmed` | BOOLEAN | 是否已确认 |

#### media（媒体文件）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `filename` | VARCHAR | 存储文件名（UUID） |
| `original_name` | VARCHAR | 原始文件名 |
| `mime_type` | VARCHAR | MIME 类型 |
| `size_bytes` | BIGINT | 文件大小 |
| `path` | VARCHAR | 存储路径 |
| `uploaded_by` | INT? | 上传者用户 ID |
| `created_at` | TIMESTAMPTZ | 上传时间 |

#### refresh_tokens（刷新令牌）

| 列名 | 类型 | 说明 |
|------|------|------|
| `id` | SERIAL PK | 主键 |
| `user_id` | INT FK | 用户 ID |
| `token` | VARCHAR | 刷新令牌（UUID） |
| `expires_at` | TIMESTAMPTZ | 过期时间 |
| `created_at` | TIMESTAMPTZ | 创建时间 |

### 4.3 关系说明

- `posts` 1:N `todo_items`（一篇文章有多个待办事项）
- `posts` 1:N `comments`（一篇文章有多个评论）
- `posts` 1:N `pull_requests`（一篇文章有多个 PR）
- `posts` 1:N `post_revisions`（一篇文章有多个修订版本）
- `posts` N:M `tags`（通过 `post_tags` 关联表）
- `pull_requests` 1:N `pr_comments`（一个 PR 有多个评论）
- `users` 1:N `refresh_tokens`（一个用户有多个刷新令牌）
- `users` 1:N `posts`（一个用户有多篇文章）
- `todo_items` 1:N `todo_subscriptions`（一个 TODO 有多个订阅者）

### 4.4 迁移管理

迁移文件存放在 `backend/src/migrations/`，使用 SeaORM 的迁移框架。命名规则：`m{YYYYMMDD}_{序号}_{描述}.rs`。

运行迁移：

```bash
cargo run -- migrate
```

迁移文件共 20 个，按时间顺序执行。每个迁移实现 `MigrationTrait`，包含 `up()` 和 `down()` 方法。

---

## 5. API 设计原则

### 5.1 RESTful 风格

所有端点以 `/api/v1/` 为前缀。资源使用复数名词（`/posts`、`/tags`、`/comments`）。HTTP 方法语义明确：

- `GET` 获取资源
- `POST` 创建资源
- `PUT` 更新资源
- `DELETE` 删除资源

### 5.2 认证机制

写操作（POST、PUT、DELETE）需要认证。读操作（GET、HEAD、OPTIONS）公开访问。部分 POST 端点（PR 创建、评论、订阅）为公开端点。

### 5.3 错误处理

所有错误返回统一 JSON 格式：

```json
{ "error": "错误描述信息" }
```

对应 HTTP 状态码：

| AppError 变体 | HTTP 状态码 |
|---------------|-------------|
| `NotFound` | 404 |
| `BadRequest` | 400 |
| `PayloadTooLarge` | 413 |
| `Internal` | 500 |
| `Database` | 500 |
| `Conflict` | 409 |

### 5.4 分页策略

列表类接口返回统一分页结构：

```json
{
  "items": [],
  "total": 100,
  "page": 1,
  "per_page": 10
}
```

- `page`：1-based 页码
- `per_page`：每页条数，范围 1-100

### 5.5 OpenAPI 文档

每个 handler 都有 `#[utoipa::path(...)]` 注解。`main.rs` 中的 `ApiDoc` 结构体汇总所有路径和 Schema。Swagger UI 在 `/swagger-ui/` 可用。

---

## 6. 认证流程

### 6.1 JWT 认证流程

```
用户注册/登录
    │
    ▼
后端验证凭据（bcrypt 校验密码）
    │
    ▼
签发 JWT access_token（15 分钟）+ refresh_token（UUID，7 天）
    │
    ▼
前端存储到 localStorage
    │
    ▼
每次请求：Axios 拦截器自动附加 Authorization: Bearer <access_token>
    │
    ▼
401 响应 → 自动用 refresh_token 换取新 token → 重试原请求
```

### 6.2 ADMIN_KEY 认证

环境变量 `ADMIN_KEY` 的值直接作为 Bearer token。中间件将请求头中的 token 与 `ADMIN_KEY` 比较。适合脚本、CI/CD、早期开发。

### 6.3 GitHub OAuth 流程

```
前端重定向到 /api/v1/auth/github
    │
    ▼
后端构造 GitHub 授权 URL，重定向用户到 GitHub
    │
    ▼
用户授权后，GitHub 回调 /api/v1/auth/github/callback
    │
    ▼
后端用授权码换取 GitHub access_token
    │
    ▼
获取 GitHub 用户资料
    │
    ▼
按 github_id 或邮箱匹配/创建用户
    │
    ▼
签发 JWT，重定向到前端 /auth/callback#access_token=...&refresh_token=...
```

### 6.4 角色权限模型

| 角色 | 权限 |
|------|------|
| `visitor` | 注册用户，可提交评论和 PR |
| `contributor` | 贡献者 |
| `admin` | 管理员，拥有全部权限 |

管理员账户在后端启动时自动创建（如果不存在）。密码使用 `ADMIN_KEY` 的值。

### 默认管理员账户

后端启动时自动创建管理员账户（如果不存在）：

| 字段 | 值 |
|------|-----|
| 邮箱 | `admin@zyblog.local` |
| 密码 | `ADMIN_KEY` 环境变量的值 |

> **本地开发默认值**：`ADMIN_KEY=dev_admin_key_change_me`（见 `.env.example`）
>
> **首次登录**：使用邮箱 `admin@zyblog.local` 和密码 `dev_admin_key_change_me` 登录。
>
> **生产环境**：务必在 `.env` 中修改 `ADMIN_KEY` 为强密码。

---

## 7. 前端架构

### 7.1 组件体系

```
App.vue
└── Layout.vue（应用外壳）
    ├── 侧边栏导航（PC）
    ├── 底部导航（移动端）
    └── <router-view>（页面内容）
        ├── Home.vue
        ├── PostDetail.vue
        ├── Publish.vue
        └── ...
```

所有组件使用 `<script setup lang="ts">`（Composition API）。UI 组件优先使用 Naive UI（`n-` 前缀）。

### 7.2 路由管理

14 条路由，全部懒加载：

| 路径 | 组件 | 说明 |
|------|------|------|
| `/` | Home.vue | 首页/文章列表 |
| `/posts` | Home.vue | 文章列表（同首页） |
| `/posts/:id` | PostDetail.vue | 文章详情 |
| `/publish` | Publish.vue | 创建文章 |
| `/publish/:id` | Publish.vue | 编辑文章 |
| `/drafts` | Drafts.vue | 草稿管理 |
| `/import` | Import.vue | 文章导入 |
| `/backup` | Backup.vue | 备份管理 |
| `/comments` | Comments.vue | 评论审核 |
| `/media` | Media.vue | 媒体管理 |
| `/todos` | Todos.vue | TODO 管理 |
| `/tags` | Tags.vue | 标签管理 |
| `/login` | Login.vue | 登录页 |
| `/github/callback` | GitHubCallback.vue | GitHub OAuth 回调 |

导航守卫保护管理员路由（`/publish`、`/drafts`、`/backup` 等）。未登录用户重定向到 `/login`。

### 7.3 状态管理 (Pinia)

单一 Pinia store `useAppStore` 管理全局状态：

- `sidebarCollapsed`：侧边栏折叠状态
- `activeRoute`：当前活动路由（菜单高亮）
- `bottomNavVisible`：移动端底部导航可见性
- `currentUser`：当前用户信息
- `isLoggedIn`：登录状态（computed）

### 7.4 Composable 模式

API 调用逻辑封装在 `composables/` 目录，组件不直接调用 API。

| Composable | 职责 |
|------------|------|
| `useAuth.ts` | 登录、注册、JWT 刷新、GitHub OAuth、用户状态 |
| `usePosts.ts` | 文章 CRUD、列表、详情 |
| `usePullRequests.ts` | PR 创建、列表、合并、评论 |
| `useRevisions.ts` | 版本列表、回滚、diff |
| `useComments.ts` | 评论提交、审核 |
| `useTags.ts` | 标签 CRUD、文章标签关联 |
| `useTodos.ts` | TODO 列表、订阅、完成 |
| `useMedia.ts` | 媒体上传、列表、删除 |
| `useSearch.ts` | 文章搜索 |
| `useResponsive.ts` | 响应式布局辅助 |

### 7.5 API 调用层

`lib/api.ts` 导出 Axios 实例，配置了两个拦截器：

**请求拦截器**：从 localStorage 读取 JWT，自动附加到请求头。排除认证端点（login、register、refresh、github）。

**响应拦截器**：401 响应时，自动用 refresh_token 换取新 token，更新 localStorage，重试原请求。使用队列机制处理并发 401。刷新失败则清除 token 并跳转登录页。

---

## 8. 后端架构

### 8.1 Handler 层

每个 handler 模块对应一个业务领域。Handler 函数签名统一：

```rust
pub async fn handler_name(
    State(state): State<Arc<AppState>>,
    // 其他提取器...
) -> Result<Json<Response>, AppError>
```

每个函数都有 `#[utoipa::path(...)]` 注解，自动生成 OpenAPI 文档。

Handler 职责一览：

| Handler | 函数 | 职责 |
|---------|------|------|
| `auth.rs` | `register` | 用户注册，bcrypt 加密密码 |
| | `login` | 用户登录，签发 JWT |
| | `refresh` | 刷新 access_token |
| | `me` | 获取当前用户信息 |
| | `github_login` | 重定向到 GitHub OAuth |
| | `github_callback` | 处理 GitHub 回调，重定向 |
| | `github_callback_json` | 处理 GitHub 回调，返回 JSON |
| `posts.rs` | `create_post` | 创建文章，提取 #todo，通知订阅者 |
| | `list_posts` | 分页列表，支持状态筛选 |
| | `get_post` | 获取单篇文章 |
| | `update_post` | 更新文章，创建修订版本快照 |
| | `delete_post` | 删除文章及关联数据 |
| | `search_posts` | 按标题和内容搜索 |
| | `get_post_todos` | 获取文章的待办事项 |
| `pulls.rs` | `create_pull` | 创建 PR，验证片段 |
| | `list_pulls` | 获取文章的 PR 列表 |
| | `update_pull` | 更新 PR 状态 |
| | `apply_pull` | 合并 PR，应用片段到文章 |
| | `add_comment` | 添加 PR 行内评论 |
| | `list_comments` | 获取 PR 评论列表 |
| `revisions.rs` | `create_revision` | 创建版本快照 |
| | `list_revisions` | 获取版本列表 |
| | `get_revision` | 获取特定版本 |
| | `rollback_revision` | 回滚到指定版本 |
| | `diff_revisions` | 比较两个版本差异 |
| `comments.rs` | `create_comment` | 提交评论（待审核） |
| | `list_comments` | 获取已审核评论 |
| | `list_pending` | 获取待审核评论 |
| | `approve_comment` | 审核通过 |
| | `delete_comment` | 删除评论 |
| `tags.rs` | `create_tag` | 创建标签 |
| | `list_tags` | 获取所有标签 |
| | `delete_tag` | 删除标签 |
| | `assign_tags_to_post` | 为文章分配标签 |
| | `remove_tag_from_post` | 移除文章标签 |
| | `list_post_tags` | 获取文章的标签 |
| `todos.rs` | `list_todos` | 获取 TODO 列表 |
| | `subscribe_todo` | 订阅 TODO |
| | `unsubscribe_todo` | 取消订阅 |
| | `complete_todo` | 标记完成，发送通知 |
| `subscribers.rs` | `create_subscriber` | 添加订阅者 |
| | `list_subscribers` | 获取订阅者列表 |
| `media.rs` | `upload_media` | 上传媒体文件 |
| | `list_media` | 获取媒体列表 |
| | `delete_media` | 删除媒体文件 |
| `images.rs` | `upload_image` | 上传图片 |
| `videos.rs` | `upload_video` | 上传视频 |
| `backup.rs` | `create_backup` | 创建备份 |
| | `list_backups` | 获取备份列表 |
| | `restore_backup` | 从备份恢复 |
| `export.rs` | `export_posts` | 导出文章（JSON/CSV） |

### 8.2 Model 层 (SeaORM)

每个模型文件定义一个 SeaORM 实体，包含：

- `Model` 结构体：表字段映射
- `Relation` 枚举：表间关系
- `Related<OtherEntity>` 实现：关联查询
- `ActiveModelBehavior` 实现：CRUD 操作

模型通过 `models/mod.rs` 统一导出。

### 8.3 Middleware 层

**`admin_auth_middleware`**（`middleware/auth.rs`）：

1. GET/HEAD/OPTIONS 请求直接放行
2. 公开 POST 端点（PR 创建、评论、订阅）放行
3. 其他请求检查 `Authorization: Bearer <token>`
4. Token 与 `ADMIN_KEY` 环境变量比较

**JWT 中间件**（`middleware/jwt.rs`）：

解析 JWT token，提取 Claims（user_id、role、exp）。用于需要识别具体用户的端点。

### 8.4 Route 层

每个路由模块定义一组相关路由，返回 `Router<Arc<AppState>>`。在 `main.rs` 中通过 `.nest()` 挂载到主路由。

路由组装顺序：

1. 直接路由（`/api/health`、`/api/v1/posts` 等）
2. 嵌套路由（`/api/v1/auth/*`、`/api/v1/backup/*` 等）
3. 静态文件服务（`/static`）
4. Body 限制中间件（105MB）
5. 认证中间件

### 8.5 Task 层（后台任务）

**邮件任务**（`tasks/email.rs`）：

- `notify_subscribers_on_update`：文章发布时通知所有已确认订阅者
- `notify_todo_subscribers`：TODO 完成时通知订阅者
- 使用 lettre 库通过 SMTP 发送邮件

**备份任务**（`tasks/backup.rs`）：

- `create_pg_dump`：调用 `pg_dump` 生成 gzip 压缩的 SQL 备份
- `list_backup_files`：列出备份目录中的文件
- `restore_from_dump`：解压并用 `psql` 恢复
- `cleanup_old_backups`：按保留数量清理旧备份
- `run_scheduled_backup`：定时备份入口，在 `main.rs` 中通过 `tokio::spawn` 启动

---

## 9. 关键模块说明

### 9.1 认证模块

**后端**：`handlers/auth.rs` + `middleware/auth.rs` + `middleware/jwt.rs`

- 注册：验证邮箱格式和密码长度，bcrypt 加密存储，签发 JWT
- 登录：验证凭据，签发 JWT
- 刷新：验证 refresh_token（一次性），签发新 access_token
- GitHub OAuth：授权码换取 token，获取用户资料，自动创建/关联账户
- 管理员初始化：启动时检查是否存在 admin 用户，不存在则用 ADMIN_KEY 创建

### 9.2 文章模块

**后端**：`handlers/posts.rs`

- 创建文章时自动从内容中提取 `#todo` 标签，创建 todo_items 记录
- 更新文章时自动创建修订版本快照
- 文章从草稿变为已发布时，自动通知订阅者
- slug 从标题自动生成（`slug::slugify()`）

### 9.3 标签模块

**后端**：`handlers/tags.rs`

- 标签 CRUD 操作
- 文章标签关联（N:M 关系，通过 post_tags 表）
- 批量分配标签，已存在的关联自动跳过

### 9.4 评论模块

**后端**：`handlers/comments.rs`

- 评论默认未审核，需要管理员审核后才对外显示
- 支持引用文章片段（`referenced_content`）
- 管理员可查看待审核评论列表

### 9.5 PR 模块

**后端**：`handlers/pulls.rs` + `utils/fragment_mapper.rs`

- PR 基于片段（Fragment）机制，精确指定修改位置（行号、列号）
- 片段验证：不重叠、不越界
- 合并 PR：将片段应用到文章内容，自动创建修订版本快照
- PR 评论：支持行内评论，精确到某个片段的某一行

### 9.6 版本控制模块

**后端**：`handlers/revisions.rs` + `utils/diff_engine.rs`

- 修订版本快照：保存文章的标题、内容、摘要、封面图
- 回滚：恢复到指定版本，创建新的修订版本记录
- Diff：比较两个版本，生成 unified diff 格式

### 9.7 媒体模块

**后端**：`handlers/media.rs` + `handlers/images.rs` + `handlers/videos.rs`

- 统一媒体上传接口（`/api/v1/media`）
- 专用图片上传（`/api/v1/images`）和视频上传（`/api/v1/videos`）
- 文件存储在 `static/` 目录，使用 UUID 文件名
- 支持 JPEG、PNG、GIF、WebP（图片）和 MP4、WebM、OGG（视频）

### 9.8 TODO 模块

**后端**：`handlers/todos.rs` + `tasks/email.rs`

- 从文章内容中自动提取 `#todo` 标签
- 用户可订阅 TODO，完成时收到邮件通知
- 管理员可标记 TODO 为已完成

---

## 10. 关键函数说明

### 10.1 Handler 函数签名模式

```rust
// 创建资源
pub async fn create_xxx(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateXxxRequest>,
) -> Result<Json<XxxResponse>, AppError>

// 列表（分页）
pub async fn list_xxx(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListXxxQuery>,
) -> Result<Json<XxxListResponse>, AppError>

// 获取单个
pub async fn get_xxx(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<XxxResponse>, AppError>

// 更新
pub async fn update_xxx(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateXxxRequest>,
) -> Result<Json<XxxResponse>, AppError>

// 删除
pub async fn delete_xxx(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError>
```

### 10.2 关键工具函数

**`utils/fragment_mapper.rs`**：

- `validate_fragments(fragments, content)`：验证片段列表不重叠、不越界
- `apply_fragments(content, fragments)`：将片段应用到文章内容，返回修改后的文本
- `map_selection_to_source(content, start_line, start_col, end_line, end_col)`：将选区映射到源文本位置

**`utils/diff_engine.rs`**：

- `generate_diff(old_text, new_text)`：生成 unified diff 格式
- `apply_diff(text, diff)`：将 diff 应用到文本

### 10.3 中间件逻辑

**`admin_auth_middleware`**：

```
请求进入
  │
  ├─ GET/HEAD/OPTIONS → 放行
  │
  ├─ POST + 公开端点 → 放行
  │   - /api/v1/posts/:id/pulls
  │   - /api/v1/pulls/:id/comments
  │   - /api/v1/subscribers
  │
  └─ 其他 → 检查 Authorization 头
      │
      ├─ 无头 → 401
      │
      ├─ 格式错误 → 401
      │
      └─ Token 与 ADMIN_KEY 比较
          │
          ├─ 匹配 → 放行
          │
          └─ 不匹配 → 401
```

---

## 11. 设计决策和权衡

### 11.1 为什么选择 Axum

Axum 是 tokio 生态的原生 HTTP 框架，类型安全的提取器系统让代码更清晰。与 tower 中间件生态无缝集成。性能优秀，API 设计简洁。

### 11.2 为什么选择 SeaORM

SeaORM 提供类型安全的查询构建器，支持异步操作，迁移系统成熟。相比 diesel，编译时间更短，API 更符合 Rust 习惯。相比 sqlx，ORM 层更完善。

### 11.3 为什么选择 Naive UI

Naive UI 是 Vue 3 原生的组件库，TypeScript 支持好，主题定制灵活。相比 Element Plus，API 设计更现代。相比 Vuetify，体积更小。

### 11.4 认证方案选择

同时支持 JWT 和 ADMIN_KEY 两种认证方式。JWT 用于正常用户登录流程，ADMIN_KEY 用于脚本和 CI/CD。这是一个务实的选择，兼顾了安全性和易用性。

### 11.5 存储方案选择

文件存储使用本地文件系统（`static/` 目录），通过 tower-http 的 ServeDir 直接提供静态文件服务。简单直接，适合单机部署。如果需要扩展到多机，可以替换为 S3 兼容存储。

### 11.6 数据库可选设计

`AppState.db` 是 `Option<DatabaseConnection>`，应用在没有数据库的情况下也能启动。这种设计允许在数据库不可用时优雅降级，也方便开发时跳过数据库配置。

### 11.7 ID 类型选择

所有主键使用 `i32` 而非 UUID。对于单机博客来说，i32 足够，且占用空间更小、排序更自然、前端处理更简单。

---

## 12. 扩展指南

### 12.1 添加新 API 端点

以添加 `GET /api/v1/categories` 为例：

**第一步：创建 Handler**

```rust
// src/handlers/categories.rs
use axum::{extract::State, Json};
use std::sync::Arc;
use crate::error::AppError;
use crate::state::AppState;

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
}

#[utoipa::path(get, path = "/api/v1/categories", responses((status = 200, body = Vec<CategoryResponse>)))]
pub async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CategoryResponse>>, AppError> {
    // 业务逻辑
    Ok(Json(vec![]))
}
```

**第二步：注册模块**

```rust
// src/handlers/mod.rs 中添加
pub mod categories;
```

**第三步：创建路由**

```rust
// src/routes/categories.rs
use axum::{Router, routing::get};
use std::sync::Arc;
use crate::state::AppState;
use crate::handlers::categories;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/api/v1/categories", get(categories::list_categories))
}
```

**第四步：注册路由模块**

```rust
// src/routes/mod.rs 中添加
pub mod categories;
```

**第五步：挂载路由**

在 `main.rs` 的路由合并处加上 `.nest("/api/v1/categories", routes::categories::routes().with_state(state.clone()))`。

**第六步：注册 OpenAPI**

在 `main.rs` 的 `ApiDoc` 结构体中添加路径和 Schema。

### 12.2 添加新页面

**第一步：创建页面组件**

```vue
<!-- src/views/Categories.vue -->
<script setup lang="ts">
import { ref, onMounted } from 'vue'

const categories = ref<string[]>([])

onMounted(async () => {
  // 加载数据
})
</script>

<template>
  <div>
    <h1>分类管理</h1>
  </div>
</template>
```

**第二步：注册路由**

```typescript
// src/router/index.ts 中添加
{
  path: '/categories',
  name: 'Categories',
  component: () => import('../views/Categories.vue'),
}
```

**第三步：添加到导航守卫**（如果是管理员页面）

```typescript
const protectedRoutes = ['/publish', '/drafts', ..., '/categories']
```

### 12.3 添加新中间件

```rust
// src/middleware/logging.rs
use axum::{body::Body, http::Request, middleware::Next, response::Response};

pub async fn request_logging(
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    tracing::info!("{} {}", method, uri);
    next.run(request).await
}
```

在 `main.rs` 中注册：

```rust
.layer(axum::middleware::from_fn(middleware::logging::request_logging))
```

### 12.4 数据库迁移

创建新的迁移文件 `src/migrations/m{YYYYMMDD}_{序号}_{描述}.rs`，实现 `MigrationTrait`：

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .col(ColumnDef::new(Categories::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Categories {
    Table,
    Id,
    Name,
}
```

在 `src/migrations/mod.rs` 中注册。

### 12.5 添加新 Composable

```typescript
// src/composables/useCategories.ts
import { ref } from 'vue'
import api from '../lib/api'

export function useCategories() {
  const categories = ref<string[]>([])
  const loading = ref(false)

  async function fetchCategories() {
    loading.value = true
    try {
      const { data } = await api.get('/api/v1/categories')
      categories.value = data
    } finally {
      loading.value = false
    }
  }

  return { categories, loading, fetchCategories }
}
```

在组件中使用：

```vue
<script setup lang="ts">
import { onMounted } from 'vue'
import { useCategories } from '../composables/useCategories'

const { categories, loading, fetchCategories } = useCategories()

onMounted(() => fetchCategories())
</script>
```

---

## 附录：环境变量

| 变量 | 必填 | 默认值 | 说明 |
|------|------|--------|------|
| `DATABASE_URL` | 否 | `postgres://postgres:postgres@localhost/zyblog` | PostgreSQL 连接字符串 |
| `ADMIN_KEY` | 是 | - | 管理员 Bearer Token |
| `SERVER_ADDR` | 否 | `0.0.0.0:8080` | 后端监听地址 |
| `RUST_LOG` | 否 | `zyblog=debug,tower_http=debug` | 日志级别 |
| `JWT_SECRET` | 否 | `change-me-in-production` | JWT 签名密钥 |
| `JWT_ACCESS_EXPIRY` | 否 | `900` | Access Token 有效期（秒） |
| `JWT_REFRESH_EXPIRY` | 否 | `604800` | Refresh Token 有效期（秒） |
| `GITHUB_CLIENT_ID` | 否 | - | GitHub OAuth Client ID |
| `GITHUB_CLIENT_SECRET` | 否 | - | GitHub OAuth Client Secret |
| `GITHUB_REDIRECT_URI` | 否 | `http://localhost:8080/api/v1/auth/github/callback` | OAuth 回调地址 |
| `SMTP_HOST` | 否 | `localhost` | SMTP 服务器 |
| `SMTP_PORT` | 否 | `587` | SMTP 端口 |
| `SMTP_USERNAME` | 否 | - | SMTP 用户名 |
| `SMTP_PASSWORD` | 否 | - | SMTP 密码 |
| `SMTP_FROM` | 否 | `noreply@zyblog.local` | 发件人地址 |
| `BACKUP_DIR` | 否 | `./backups` | 备份目录 |
| `BACKUP_INTERVAL_HOURS` | 否 | `24` | 备份间隔（小时） |
| `BACKUP_RETENTION_COUNT` | 否 | `10` | 备份保留数量 |
| `VITE_API_BASE_URL` | 否 | `http://localhost:8080` | 前端 API 基础 URL |
