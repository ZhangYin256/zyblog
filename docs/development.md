# 开发指南

## 环境配置

### 前置要求

- Docker + Docker Compose
- （可选）Rust 1.75+ 和 cargo
- （可选）Node.js 20+ 和 npm

### 快速开始

```bash
# 启动所有服务
cp .env.example .env
# 编辑 .env 文件配置你的设置
docker-compose up -d
make migrate
```

### 运行测试

```bash
make test
```

## 架构设计

### 后端 (Axum)

后端结构如下：

```
src/
  main.rs          # 入口文件，路由配置，OpenAPI 配置
  config.rs        # 环境变量加载
  state.rs         # 共享应用状态
  error.rs         # 错误类型和 HTTP 响应
  handlers/        # 请求处理器（业务逻辑）
    posts.rs       # 博客文章 CRUD + #todo 解析
    subscribers.rs # 邮件订阅者管理
    export.rs      # 数据导出 (JSON/CSV)
    images.rs      # 图片上传/服务
    pulls.rs       # PR 式互动功能
    backup.rs      # 数据备份和恢复
    videos.rs      # 视频上传
  routes/          # Axum 路由定义
    posts.rs
    subscribers.rs
    export.rs
    images.rs
    pulls.rs
    backup.rs
    videos.rs
  middleware/
    auth.rs        # Bearer token 认证
  models/          # SeaORM 实体定义
    post.rs
    subscriber.rs
    todo_item.rs
    image.rs
    pull_request.rs
    pull_request_comment.rs
  migrations/      # 数据库迁移
  tasks/
    email.rs       # 邮件通知任务
    backup.rs      # 数据库备份任务
```

### 数据库

使用 PostgreSQL 配合 SeaORM。迁移通过 `cargo run -- migrate` 运行。

数据表：
- `posts` - 博客文章，包含标题、slug、内容、发布状态
- `subscribers` - 邮件订阅者
- `todo_items` - 从文章内容中的 `#todo` 标签解析出的待办事项
- `images` - 上传图片的元数据
- `pull_requests` - PR 式互动，包含文章关联、用户邮箱、内容、状态
- `pull_request_comments` - PR 评论，包含 PR 关联、用户邮箱、内容

### 认证

管理员写入操作使用 Bearer token 认证。Token 与 `ADMIN_KEY` 环境变量进行比较。读取操作 (GET) 是公开的。

### #todo 系统

文章内容中可以包含 `#todo` 标签。当文章创建或更新时，系统会：

1. 从内容中解析所有 `#todo 任务描述` 模式
2. 创建与文章关联的 `todo_item` 记录
3. 更新时，旧的待办事项会被新解析的待办事项替换

## 添加新端点

1. 在 `src/handlers/your_module.rs` 中创建处理器
2. 在 `src/handlers/mod.rs` 中添加 `pub mod your_module;`
3. 在 `src/routes/your_module.rs` 中创建路由
4. 在 `src/routes/mod.rs` 中添加 `pub mod your_module;`
5. 在 `src/main.rs` 中连接路由
6. 为 OpenAPI 文档添加 utoipa 注解
7. 在 `ApiDoc` 结构体中注册路径和模式

## 新功能开发指南

### PR 式互动功能

PR 式互动允许读者对文章提交修改建议。

**数据模型**：
- `pull_request`：存储 PR 信息，包括 `post_id`、`user_email`、`content`、`status`（open/closed/merged）
- `pull_request_comment`：存储 PR 评论，包括 `pull_request_id`、`user_email`、`content`

**API 端点**：
- `GET /api/v1/posts/:id/pulls` - 获取文章的 PR 列表
- `POST /api/v1/posts/:id/pulls` - 创建新 PR
- `PUT /api/v1/pulls/:id` - 更新 PR 状态（合并/关闭）
- `POST /api/v1/pulls/:id/comments` - 添加评论

**实现要点**：
- PR 状态必须是 `open`、`closed` 或 `merged` 之一
- 创建 PR 时需要验证文章存在
- 评论内容不能为空

### 数据备份功能

数据备份功能提供数据库的备份和恢复能力。

**依赖**：
- 需要 `pg_dump` 和 `pg_restore` 工具
- 备份目录通过 `BACKUP_DIR` 环境变量配置

**API 端点**：
- `POST /api/v1/backup` - 创建备份
- `GET /api/v1/backup/list` - 获取备份列表
- `POST /api/v1/backup/restore` - 从备份恢复

**安全考虑**：
- 所有备份操作都需要管理员认证
- 恢复操作会验证文件名，防止路径遍历攻击
- 备份文件存储在受保护的目录中

### 视频上传功能

视频上传功能支持上传视频文件到服务器。

**支持格式**：
- MP4 (video/mp4)
- WebM (video/webm)
- OGG (video/ogg)

**限制**：
- 最大文件大小：100MB
- 文件名自动生成 UUID
- 上传目录：`static/videos/`

**实现细节**：
- 使用 multipart 表单上传
- 分块读取文件避免内存溢出
- 验证文件类型和大小
- 返回可访问的 URL

## OpenAPI 文档

API 文档使用 [utoipa](https://docs.rs/utoipa) 从代码自动生成。

- 为请求/响应类型添加 `#[derive(utoipa::ToSchema)]`
- 为处理器函数添加 `#[utoipa::path(...)]`
- 在 `main.rs` 的 `ApiDoc` 结构体中注册路径和组件

Swagger UI 在 `/swagger-ui/` 提供服务，原始 OpenAPI JSON 在 `/api-docs/openapi.json`。

## 环境变量

查看 `.env.example` 了解所有可用的配置选项。