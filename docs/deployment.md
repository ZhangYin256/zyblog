# ZYBlog 部署指南

使用 Docker Compose 部署 ZYBlog 的完整流程。

## 前置要求

- Docker 20.10+
- Docker Compose 2.0+
- Git

## 快速部署

### 1. 克隆仓库

```bash
git clone https://github.com/your-username/zyblog.git
cd zyblog
```

### 2. 配置环境变量

```bash
cp .env.example .env
```

编辑 `.env`，至少修改 `ADMIN_KEY`：

```env
ADMIN_KEY=your_secure_admin_key_here
POSTGRES_PASSWORD=your_secure_db_password
```

### 3. 启动服务

```bash
docker-compose up -d
```

首次启动会构建镜像，耗时较长。后续启动直接使用缓存。

### 4. 运行数据库迁移

```bash
make migrate
```

等价于：

```bash
docker-compose exec backend cargo run -- migrate
```

### 5. 访问应用

| 服务 | 地址 |
|------|------|
| 前端页面 | http://localhost:5173 |
| 后端 API | http://localhost:8080 |
| Swagger 文档 | http://localhost:8080/swagger-ui/ |
| 数据库 | localhost:5432 |

## 环境变量说明

### 数据库

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `POSTGRES_USER` | 数据库用户名 | `zyblog` |
| `POSTGRES_PASSWORD` | 数据库密码 | `zyblog_dev` |
| `POSTGRES_DB` | 数据库名称 | `zyblog` |
| `DATABASE_URL` | PostgreSQL 连接字符串 | `postgres://zyblog:zyblog_dev@postgres:5432/zyblog` |

### 后端

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `ADMIN_KEY` | 管理员 API 密钥（Bearer Token） | `dev_admin_key_change_me` |
| `RUST_LOG` | 日志级别 | `debug` |

### 邮件通知（可选）

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `SMTP_HOST` | SMTP 服务器地址 | 空（不发送邮件） |
| `SMTP_PORT` | SMTP 端口 | `587` |
| `SMTP_USERNAME` | SMTP 用户名 | 空 |
| `SMTP_PASSWORD` | SMTP 密码 | 空 |
| `SMTP_FROM` | 发件人地址 | `noreply@zyblog.local` |

### 前端

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `VITE_API_BASE_URL` | 后端 API 地址 | `http://localhost:8080` |

> 生产环境部署时，务必修改 `ADMIN_KEY` 和 `POSTGRES_PASSWORD`。默认值仅用于本地开发。

## 服务说明

### postgres

PostgreSQL 15 数据库，数据持久化到 Docker 命名卷 `postgres_data`。

- 镜像：`postgres:15-alpine`
- 端口：5432
- 健康检查：每 5 秒通过 `pg_isready` 检测连接

### backend

Axum (Rust) API 服务器，使用 cargo-chef 多阶段构建，最终镜像基于 `debian:bookworm-slim`。

- 构建阶段：chef -> planner -> builder -> runtime
- 端口：8080
- 依赖 postgres 健康检查通过后才启动

### frontend

Vue 3 静态文件，通过 nginx 提供服务，内置 SPA 路由和 API 反向代理。

- 构建阶段：`node:20-alpine` 编译，`nginx:alpine` 运行
- 端口：80（容器内）
- 反向代理：`/api/`、`/static/`、`/swagger-ui/`、`/api-docs/` 均转发到 backend:8080

## 常用命令

所有命令均可通过 Makefile 简化调用：

| 命令 | 说明 |
|------|------|
| `make up` | 启动所有服务（后台运行） |
| `make down` | 停止所有服务 |
| `make restart` | 重启所有服务 |
| `make build` | 重新构建镜像 |
| `make build-no-cache` | 无缓存重新构建 |
| `make logs` | 查看所有服务日志（实时） |
| `make logs-backend` | 查看后端日志 |
| `make logs-frontend` | 查看前端日志 |
| `make logs-postgres` | 查看数据库日志 |
| `make migrate` | 运行数据库迁移 |
| `make test` | 运行后端测试 |
| `make db-shell` | 进入 psql 命令行 |
| `make ps` | 查看运行中的容器 |
| `make clean` | 删除所有容器和数据卷 |
| `make env` | 从 `.env.example` 生成 `.env` |

## 数据备份

ZYBlog 内置数据库备份功能。

- 备份目录：`backend/backups/`（挂载在容器内）
- 自动备份：服务启动后每 24 小时执行一次
- 手动备份：调用 `POST /api/v1/backup`（需要认证）

管理备份的 API：

```bash
# 创建备份
curl -X POST -H "Authorization: Bearer YOUR_ADMIN_KEY" \
  http://localhost:8080/api/v1/backup

# 查看备份列表
curl -H "Authorization: Bearer YOUR_ADMIN_KEY" \
  http://localhost:8080/api/v1/backup/list

# 从备份恢复
curl -X POST -H "Authorization: Bearer YOUR_ADMIN_KEY" \
  -H "Content-Type: application/json" \
  -d '{"filename": "backup_20240101_120000.sql"}' \
  http://localhost:8080/api/v1/backup/restore
```

## 常见问题

### 端口被占用

如果 5432、8080 或 5173 端口已被其他进程占用，修改 `docker-compose.yml` 中的端口映射：

```yaml
ports:
  - "15432:5432"  # 将外部端口改为 15432
```

或者停用占用端口的进程：

```bash
# 查看占用端口的进程
lsof -i :5432
lsof -i :8080
lsof -i :5173
```

### 数据库连接失败

后端依赖 postgres 健康检查通过后才会启动。如果仍然连接失败：

1. 确认 postgres 容器正在运行：`make ps`
2. 查看 postgres 日志：`make logs-postgres`
3. 确认 `.env` 中 `DATABASE_URL` 的主机名为 `postgres`（不是 `localhost`）

容器内访问数据库用 `postgres` 作为主机名，宿主机访问用 `localhost`。

### 迁移失败

```bash
# 查看后端日志定位错误
make logs-backend

# 手动进入数据库检查表结构
make db-shell
```

### 构建缓慢

首次构建需要编译 Rust 依赖，耗时较长。后续构建会利用 cargo-chef 的缓存层加速。

如需完全重新构建：

```bash
make build-no-cache
```

### 清理所有数据

删除所有容器、数据卷和缓存：

```bash
make clean
```

> 警告：这会删除数据库中的所有数据，操作前请确认已备份。

### 查看容器状态

```bash
make ps
```

正常运行时应看到三个容器均为 `Up` 状态。
