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
  routes/          # Axum 路由定义
    posts.rs
    subscribers.rs
    export.rs
    images.rs
  middleware/
    auth.rs        # Bearer token 认证
  models/          # SeaORM 实体定义
    post.rs
    subscriber.rs
    todo_item.rs
    image.rs
  migrations/      # 数据库迁移
  tasks/
    email.rs       # 邮件通知任务
```

### 数据库

使用 PostgreSQL 配合 SeaORM。迁移通过 `cargo run -- migrate` 运行。

数据表：
- `posts` - 博客文章，包含标题、slug、内容、发布状态
- `subscribers` - 邮件订阅者
- `todo_items` - 从文章内容中的 `#todo` 标签解析出的待办事项
- `images` - 上传图片的元数据

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

## OpenAPI 文档

API 文档使用 [utoipa](https://docs.rs/utoipa) 从代码自动生成。

- 为请求/响应类型添加 `#[derive(utoipa::ToSchema)]`
- 为处理器函数添加 `#[utoipa::path(...)]`
- 在 `main.rs` 的 `ApiDoc` 结构体中注册路径和组件

Swagger UI 在 `/swagger-ui/` 提供服务，原始 OpenAPI JSON 在 `/api-docs/openapi.json`。

## 环境变量

查看 `.env.example` 了解所有可用的配置选项。