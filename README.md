# ZYBlog

一个使用 Vue 3 + Axum (Rust) + PostgreSQL + SeaORM 构建的现代化博客平台。

## 技术栈

- **前端**: Vue 3 + Vite
- **后端**: Axum (Rust)
- **数据库**: PostgreSQL 15
- **ORM**: SeaORM
- **API 文档**: utoipa (OpenAPI 3.0) + Swagger UI

## 快速开始

### 前置要求

- Docker 和 Docker Compose
- （可选）Rust 工具链，用于本地开发

### 使用 Docker

```bash
# 克隆仓库
git clone <repo-url>
cd zyblog

# 复制环境变量文件
cp .env.example .env

# 编辑 .env 设置你的 ADMIN_KEY 和其他配置
# vim .env

# 启动所有服务
docker-compose up -d

# 运行数据库迁移
make migrate
```

应用将在以下地址可用：

- **前端**: http://localhost:5173
- **后端 API**: http://localhost:8080
- **Swagger UI**: http://localhost:8080/swagger-ui/

### 本地开发

```bash
# 仅启动数据库
docker-compose up -d postgres

# 设置环境变量
export DATABASE_URL=postgres://zyblog:zyblog_dev@localhost:5432/zyblog
export ADMIN_KEY=your_admin_key

# 运行后端
cd backend
cargo run

# 运行前端（在另一个终端）
cd frontend
npm install
npm run dev
```

## 环境变量

| 变量 | 说明 | 默认值 |
|---|---|---|
| `DATABASE_URL` | PostgreSQL 连接字符串 | `postgres://zyblog:zyblog_dev@postgres:5432/zyblog` |
| `ADMIN_KEY` | 管理员 API 访问的 Bearer 令牌 | （必填） |
| `SERVER_ADDR` | 后端监听地址 | `0.0.0.0:8080` |
| `RUST_LOG` | 日志级别 | `zyblog=debug,tower_http=debug` |
| `SMTP_HOST` | 邮件通知的 SMTP 服务器 | （空） |
| `SMTP_PORT` | SMTP 端口 | `587` |
| `SMTP_USERNAME` | SMTP 用户名 | （空） |
| `SMTP_PASSWORD` | SMTP 密码 | （空） |
| `SMTP_FROM` | 发件人邮箱地址 | `noreply@zyblog.local` |
| `VITE_API_BASE_URL` | 前端 API 基础 URL | `http://localhost:8080` |

## API 概览

所有端点均以 `/api/v1` 为前缀。

### 文章

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/v1/posts` | 获取文章列表（分页） |
| `POST` | `/api/v1/posts` | 创建文章（需要认证） |
| `GET` | `/api/v1/posts/:id` | 获取单篇文章 |
| `PUT` | `/api/v1/posts/:id` | 更新文章（需要认证） |
| `DELETE` | `/api/v1/posts/:id` | 删除文章（需要认证） |
| `GET` | `/api/v1/posts/:id/todos` | 获取文章的待办事项 |

### 订阅者

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/v1/subscribers` | 获取订阅者列表 |
| `POST` | `/api/v1/subscribers` | 添加订阅者 |

### 导出

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/v1/export/posts` | 导出文章为 JSON 或 CSV（需要认证） |

### 其他

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/health` | 健康检查 |
| `GET` | `/swagger-ui/` | Swagger UI |
| `GET` | `/api-docs/openapi.json` | OpenAPI 规范 |

服务器运行时，完整的交互式文档可在 `/swagger-ui/` 访问。

## 认证

写操作（POST、PUT、DELETE）需要在 `Authorization` 请求头中携带 Bearer 令牌：

```bash
curl -H "Authorization: Bearer YOUR_ADMIN_KEY" \
     -X POST http://localhost:8080/api/v1/posts \
     -H "Content-Type: application/json" \
     -d '{"title": "Hello", "content": "World"}'
```

读操作（GET）是公开的，不需要认证。

## 数据导出

以 JSON 或 CSV 格式导出所有博客文章：

```bash
# JSON 导出
curl -H "Authorization: Bearer YOUR_ADMIN_KEY" \
     http://localhost:8080/api/v1/export/posts \
     -o posts.json

# CSV 导出
curl -H "Authorization: Bearer YOUR_ADMIN_KEY" \
     "http://localhost:8080/api/v1/export/posts?format=csv" \
     -o posts.csv

# 按状态筛选
curl -H "Authorization: Bearer YOUR_ADMIN_KEY" \
     "http://localhost:8080/api/v1/export/posts?status=published" \
     -o published.json
```

## Makefile 命令

| 命令 | 说明 |
|---|---|
| `make up` | 启动所有服务 |
| `make down` | 停止所有服务 |
| `make logs` | 查看所有日志 |
| `make logs-backend` | 查看后端日志 |
| `make build` | 构建所有服务 |
| `make test` | 运行后端测试 |
| `make migrate` | 运行数据库迁移 |
| `make db-shell` | 打开 psql 命令行 |
| `make clean` | 删除卷和容器 |

## 项目结构

```
zyblog/
  backend/           # Axum (Rust) 后端
    src/
      handlers/      # 请求处理器
      middleware/     # 认证中间件
      models/        # SeaORM 实体模型
      routes/        # 路由定义
      migrations/    # 数据库迁移
      tasks/         # 后台任务（邮件）
    static/          # 静态文件（上传的图片）
  frontend/          # Vue 3 前端
  docker-compose.yml
  Makefile
```

## 许可证

MIT
