# ZYBlog 开发指南

本指南帮助新开发者快速上手 ZYBlog 项目。涵盖环境搭建、本地启动、开发流程和代码规范。

## 环境要求

| 工具 | 最低版本 | 用途 |
|------|----------|------|
| Rust | 1.75+ | 后端编译和运行 |
| Node.js | 20+ | 前端构建和开发 |
| PostgreSQL | 15+ | 数据库 |
| cargo | 随 Rust 安装 | Rust 包管理 |
| npm | 随 Node.js 安装 | 前端包管理 |

可选工具：
- Docker + Docker Compose（容器化部署，不装 PostgreSQL 也能跑）
- tmux（方便同时管理多个终端）

## 快速开始

### 1. 克隆仓库

```bash
git clone <repo-url> zyblog
cd zyblog
```

### 2. 安装并启动 PostgreSQL

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install postgresql postgresql-contrib
sudo service postgresql start

# macOS (Homebrew)
brew install postgresql@15
brew services start postgresql@15
```

### 3. 创建数据库

```bash
sudo -u postgres psql
```

```sql
CREATE USER zyblog WITH PASSWORD 'zyblog_dev';
CREATE DATABASE zyblog OWNER zyblog;
\q
```

### 4. 运行数据库迁移

```bash
cd backend
DATABASE_URL=postgres://zyblog:zyblog_dev@localhost:5432/zyblog cargo run -- migrate
```

### 5. 启动后端

```bash
# 在 backend/ 目录下
DATABASE_URL=postgres://zyblog:zyblog_dev@localhost:5432/zyblog \
  ADMIN_KEY=your_admin_key_here \
  cargo run
```

后端默认监听 `0.0.0.0:8080`。

### 6. 启动前端

```bash
# 新开一个终端，在 frontend/ 目录下
cd frontend
npm install
npm run dev
```

前端默认运行在 `http://localhost:5173`，自动代理 `/api` 请求到后端。

### 7. 验证

- 前端页面：http://localhost:5173
- 后端 API：http://localhost:8080/api/health
- API 文档：http://localhost:8080/swagger-ui/

### Docker 方式（可选）

不想装 PostgreSQL 的话，可以用 Docker：

```bash
cp .env.example .env
docker-compose up -d
make migrate
```

`.env` 文件中配置 `ADMIN_KEY`、数据库密码等。更多 Makefile 命令见下方。

## 项目结构

```
zyblog/
├── backend/                # Axum (Rust) 后端
│   ├── src/
│   │   ├── main.rs         # 入口：路由组装、OpenAPI、备份调度
│   │   ├── lib.rs          # 模块导出（供集成测试使用）
│   │   ├── config.rs       # 环境变量加载（dotenvy）
│   │   ├── state.rs        # AppState 共享状态
│   │   ├── error.rs        # AppError 错误类型
│   │   ├── handlers/       # 请求处理器（业务逻辑）
│   │   ├── models/         # SeaORM 实体定义
│   │   ├── routes/         # 路由分组
│   │   ├── middleware/     # 认证中间件
│   │   ├── migrations/     # 数据库迁移文件
│   │   └── tasks/          # 后台任务（邮件、备份）
│   ├── tests/              # 集成测试
│   └── static/             # 上传的图片和视频
├── frontend/               # Vue 3 前端
│   └── src/
│       ├── main.ts         # 应用入口
│       ├── App.vue         # 根组件
│       ├── router/         # 路由定义
│       ├── stores/         # Pinia 状态管理
│       ├── lib/            # Axios 实例配置
│       ├── composables/    # 组合式函数（API 调用）
│       ├── components/     # 可复用组件
│       ├── views/          # 页面组件
│       └── __tests__/      # 测试文件
├── docs/                   # 项目文档
├── docker-compose.yml      # 容器编排
├── Makefile                # Docker 快捷命令
└── run.txt                 # 本地开发启动命令
```

## 后端开发

### 技术栈

- **框架**：Axum 0.7（异步 HTTP 框架）
- **ORM**：SeaORM 1.x（支持 PostgreSQL）
- **序列化**：serde + serde_json
- **错误处理**：thiserror（AppError 枚举）
- **日志**：tracing + tracing-subscriber
- **API 文档**：utoipa（自动生成 OpenAPI 3.0）
- **邮件**：lettre（SMTP）
- **运行时**：tokio（全功能）

### 添加新端点

以添加 `GET /api/v1/tags` 为例：

**第一步：创建处理器**

```rust
// src/handlers/tags.rs
use axum::Json;
use crate::error::AppError;

#[utoipa::path(
    get,
    path = "/api/v1/tags",
    responses((status = 200, body = Vec<String>))
)]
pub async fn list_tags() -> Result<Json<Vec<String>>, AppError> {
    // 业务逻辑
    Ok(Json(vec![]))
}
```

**第二步：注册模块**

```rust
// src/handlers/mod.rs 中添加
pub mod tags;
```

**第三步：创建路由**

```rust
// src/routes/tags.rs
use axum::{Router, routing::get};
use std::sync::Arc;
use crate::state::AppState;
use crate::handlers::tags;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/api/v1/tags", get(tags::list_tags))
}
```

**第四步：注册路由模块**

```rust
// src/routes/mod.rs 中添加
pub mod tags;
```

**第五步：挂载路由**

在 `src/main.rs` 的路由合并处加上 `.merge(routes::tags::routes())`。

**第六步：注册 OpenAPI**

在 `main.rs` 的 `ApiDoc` 结构体中添加路径：

```rust
#[derive(OpenApi)]
#[openapi(paths(tags::list_tags))]
struct ApiDoc;
```

### 添加新数据库模型

以添加 `tags` 表为例：

**第一步：生成迁移文件**

```bash
# SeaORM 迁移文件放在 src/migrations/ 下
# 新建一个迁移文件，参考已有的迁移格式
```

**第二步：定义实体**

```rust
// src/models/tag.rs
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

**第三步：注册实体模块**

```rust
// src/models/mod.rs 中添加
pub mod tag;
```

### 运行测试

```bash
cd backend

# 运行所有测试（单元 + 集成）
cargo test

# 显示 println 输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_create_post

# 检查编译
cargo check
```

测试注意事项：
- 集成测试在 `tests/` 目录，通过 `use zyblog::...` 导入
- 单元测试直接写在各模块的 `#[cfg(test)]` 块中
- 测试用 `tempfile` 创建临时文件，不会污染项目目录

### 后端常用命令

```bash
cargo run                    # 启动服务器
cargo run -- migrate         # 只运行迁移
cargo check                  # 检查编译（比 build 快）
cargo clippy                 # 代码检查
cargo fmt                    # 格式化代码
```

## 前端开发

### 技术栈

- **框架**：Vue 3.5（Composition API）
- **构建工具**：Vite 5
- **状态管理**：Pinia 3
- **路由**：Vue Router 5
- **UI 组件库**：Naive UI
- **HTTP 客户端**：Axios
- **语言**：TypeScript（严格模式）
- **测试**：Vitest + happy-dom + @vue/test-utils

### 添加新页面

以添加 `/tags` 页面为例：

**第一步：创建页面组件**

```vue
<!-- src/views/Tags.vue -->
<script setup lang="ts">
import { ref, onMounted } from 'vue'

const tags = ref<string[]>([])

onMounted(async () => {
  // 加载数据
})
</script>

<template>
  <div>
    <h1>标签管理</h1>
    <!-- 页面内容 -->
  </div>
</template>
```

**第二步：注册路由**

```typescript
// src/router/index.ts 中添加
{
  path: '/tags',
  name: 'Tags',
  component: () => import('../views/Tags.vue'),  // 懒加载
}
```

### 添加新 Composable

API 调用逻辑放在 `composables/` 目录，不要写在组件里。

```typescript
// src/composables/useTags.ts
import { ref } from 'vue'
import api from '../lib/api'

export function useTags() {
  const tags = ref<string[]>([])
  const loading = ref(false)

  async function fetchTags() {
    loading.value = true
    try {
      const { data } = await api.get('/api/v1/tags')
      tags.value = data
    } finally {
      loading.value = false
    }
  }

  return { tags, loading, fetchTags }
}
```

在组件中使用：

```vue
<script setup lang="ts">
import { onMounted } from 'vue'
import { useTags } from '../composables/useTags'

const { tags, loading, fetchTags } = useTags()

onMounted(() => fetchTags())
</script>
```

### 添加新组件

```vue
<!-- src/components/TagBadge.vue -->
<script setup lang="ts">
defineProps<{
  name: string
}>()
</script>

<template>
  <n-tag>{{ name }}</n-tag>
</template>
```

组件放 `components/` 目录，页面级组件放 `views/` 目录。

### 运行测试

```bash
cd frontend

# 单次运行所有测试
npm run test

# 监听模式（文件修改自动重跑）
npm run test:watch

# 带覆盖率报告
npm run test:coverage

# 类型检查 + 构建
npm run build
```

测试文件放在 `src/__tests__/` 下，按类型分目录：
- `__tests__/views/` 页面测试
- `__tests__/components/` 组件测试
- `__tests__/composables/` 组合式函数测试

### 前端常用命令

```bash
npm run dev          # 启动开发服务器（端口 5173）
npm run build        # 类型检查 + 生产构建
npm run test         # 运行测试
npm run preview      # 预览构建产物
```

## 代码规范

### Rust

- 使用标准 `rustfmt` 格式化（`cargo fmt`）
- 使用 `clippy` 检查代码质量（`cargo clippy`）
- 错误处理用 `?` 操作符和 `AppError`，不要用 `unwrap()`
- 日志用 `tracing::info!`、`tracing::error!` 等，不要用 `println!` 或 `dbg!!`
- 数据库访问必须通过 `state.db.as_ref().ok_or(...)`，因为 DB 连接是可选的
- 每个处理器函数都要加 `#[utoipa::path(...)]` 注解
- ID 统一用 `i32`，时间戳用 `chrono::DateTime<Utc>`

### TypeScript

- 严格模式（`tsconfig.json` 中 `"strict": true`）
- 不要用 `any` 类型
- 用 `interface` 或 `type` 定义数据结构
- API 返回值要有类型定义

### Vue 组件

- 全部使用 Composition API + `<script setup lang="ts">`
- 不要用 Options API
- API 调用逻辑放 `composables/`，不要写在组件里
- UI 组件优先用 Naive UI（`n-` 前缀组件）
- 路由全部懒加载：`() => import('../views/...')`

## 常见问题

### 后端启动报错 "connection refused"

检查 PostgreSQL 是否在运行：

```bash
sudo service postgresql status
# 如果没运行
sudo service postgresql start
```

确认 `DATABASE_URL` 中的用户名、密码、数据库名是否正确。

### 前端请求返回 404

开发模式下 Vite 会把 `/api` 请求代理到 `http://localhost:8080`。如果后端没启动，请求会失败。确保后端先启动。

### 迁移失败

```bash
# 查看当前迁移状态
DATABASE_URL=postgres://zyblog:zyblog_dev@localhost:5432/zyblog cargo run -- migrate

# 如果数据库损坏，重建
sudo -u postgres psql
DROP DATABASE zyblog;
CREATE DATABASE zyblog OWNER zyblog;
\q
# 然后重新运行迁移
```

### cargo 编译很慢

首次编译需要下载和编译所有依赖，可能要几分钟。后续增量编译会快很多。

```bash
# 只检查不编译（更快）
cargo check
```

### npm install 报错

```bash
# 清除缓存重试
rm -rf node_modules package-lock.json
npm install
```

### Docker 方式相关

```bash
# 查看所有服务状态
make ps

# 查看后端日志
make logs-backend

# 进入数据库命令行
make db-shell

# 停止并清理
make clean
```

### ADMIN_KEY 怎么设

`ADMIN_KEY` 是管理员 API 的 Bearer Token，随便设一个字符串就行，开发环境用什么都无所谓：

```bash
ADMIN_KEY=dev123 cargo run
```

前端登录时输入同样的值。Token 存在浏览器 localStorage 中。

### 视频上传失败

检查文件大小限制。后端设置了 105MB 的 body 限制。支持的格式：MP4、WebM、OGG。

### Swagger UI 打不开

确保后端正在运行，访问 http://localhost:8080/swagger-ui/（注意末尾的斜杠）。
