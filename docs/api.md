# ZYBlog API 文档

**版本**: 0.1.0  
**基础 URL**: `http://localhost:8080`  
**交互式文档**: `http://localhost:8080/swagger-ui/`  
**OpenAPI 规范**: `http://localhost:8080/api-docs/openapi.json`

---

## 认证

系统支持两种认证方式，写操作（POST、PUT、DELETE）必须携带认证信息，读操作（GET、HEAD、OPTIONS）公开访问。

### 方式一：JWT Bearer Token

用户通过注册或登录获取 JWT token，包含 `access_token`（15 分钟有效）和 `refresh_token`（7 天有效）。

```
Authorization: Bearer <access_token>
```

### 方式二：ADMIN_KEY

管理员可直接使用环境变量 `ADMIN_KEY` 的值作为 Bearer token。适合脚本和 CI/CD 场景。

```
Authorization: Bearer <ADMIN_KEY>
```

### 角色体系

| 角色 | 权限说明 |
|------|----------|
| `visitor` | 注册用户，可提交评论和 PR |
| `contributor` | 贡献者 |
| `admin` | 管理员，拥有全部权限 |

---

## 错误格式

所有错误返回统一 JSON 格式：

```json
{
  "error": "错误描述信息"
}
```

### 常见 HTTP 状态码

| 状态码 | 含义 |
|--------|------|
| 200 | 成功 |
| 201 | 创建成功 |
| 204 | 删除成功（无响应体） |
| 400 | 请求参数错误 |
| 401 | 未认证或 token 无效 |
| 404 | 资源不存在 |
| 409 | 资源冲突（如邮箱已注册） |
| 413 | 文件过大 |
| 500 | 服务器内部错误 |

---

## 分页格式

列表类接口返回统一分页结构：

```json
{
  "items": [],
  "total": 100,
  "page": 1,
  "per_page": 10
}
```

---

## 端点列表

### 认证 (Auth)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| POST | `/api/v1/auth/register` | 无需 | 注册新用户 |
| POST | `/api/v1/auth/login` | 无需 | 用户登录 |
| POST | `/api/v1/auth/refresh` | 无需 | 刷新 access_token |
| GET | `/api/v1/auth/me` | JWT 或 ADMIN_KEY | 获取当前用户信息 |
| GET | `/api/v1/auth/github` | 无需 | GitHub OAuth 登录（重定向） |
| GET | `/api/v1/auth/github/callback` | 无需 | GitHub OAuth 回调（重定向） |
| GET | `/api/v1/auth/github/token` | 无需 | GitHub OAuth 回调（返回 JSON） |

### 文章 (Posts)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| GET | `/api/v1/posts` | 无需 | 获取文章列表（分页） |
| POST | `/api/v1/posts` | 需要 | 创建文章 |
| GET | `/api/v1/posts/search?q=` | 无需 | 搜索文章 |
| GET | `/api/v1/posts/:id` | 无需 | 获取单篇文章 |
| PUT | `/api/v1/posts/:id` | 需要 | 更新文章 |
| DELETE | `/api/v1/posts/:id` | 需要 | 删除文章 |
| GET | `/api/v1/posts/:id/todos` | 无需 | 获取文章的待办事项 |

### 标签 (Tags)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| GET | `/api/v1/tags` | 无需 | 获取所有标签 |
| POST | `/api/v1/tags` | 需要 | 创建标签 |
| DELETE | `/api/v1/tags/:id` | 需要 | 删除标签 |
| GET | `/api/v1/posts/:id/tags` | 无需 | 获取文章的标签 |
| POST | `/api/v1/posts/:id/tags` | 需要 | 为文章分配标签 |
| DELETE | `/api/v1/posts/:id/tags/:tag_id` | 需要 | 移除文章的标签 |

### 评论 (Comments)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| POST | `/api/v1/posts/:id/comments` | 无需 | 提交评论（待审核） |
| GET | `/api/v1/posts/:id/comments` | 无需 | 获取文章已审核评论 |
| GET | `/api/v1/comments/pending` | 需要 | 获取待审核评论列表 |
| PUT | `/api/v1/comments/:id/approve` | 需要 | 审核通过评论 |
| DELETE | `/api/v1/comments/:id` | 需要 | 删除评论 |

### 版本控制 (Revisions)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| POST | `/api/v1/posts/:id/revisions` | 需要 | 创建修订版本快照 |
| GET | `/api/v1/posts/:id/revisions` | 无需 | 获取修订版本列表 |
| GET | `/api/v1/posts/:id/revisions/:rev_id` | 无需 | 获取特定修订版本 |
| POST | `/api/v1/posts/:id/revisions/:rev_id/rollback` | 需要 | 回滚到指定版本 |
| GET | `/api/v1/posts/:id/diff?from=&to=` | 无需 | 比较两个版本差异 |

### PR (Pull Requests)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| POST | `/api/v1/posts/:id/pulls` | 无需 | 创建 PR |
| GET | `/api/v1/posts/:id/pulls` | 无需 | 获取文章的 PR 列表 |
| PUT | `/api/v1/pulls/:id` | 需要 | 更新 PR 状态 |
| POST | `/api/v1/pulls/:id/apply` | 需要 | 合并 PR |
| POST | `/api/v1/pulls/:id/comments` | 无需 | 添加 PR 行内评论 |
| GET | `/api/v1/pulls/:id/comments` | 无需 | 获取 PR 评论列表 |

### 媒体 (Media)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| POST | `/api/v1/media` | 需要 | 上传媒体文件（图片/视频） |
| GET | `/api/v1/media` | 无需 | 获取媒体文件列表（分页） |
| DELETE | `/api/v1/media/:id` | 需要 | 删除媒体文件 |

### 图片 (Images)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| POST | `/api/v1/images` | 需要 | 上传图片 |

### 视频 (Videos)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| POST | `/api/v1/videos` | 需要 | 上传视频 |

### TODO

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| GET | `/api/v1/todos` | 无需 | 获取 TODO 列表 |
| POST | `/api/v1/todos/:id/subscribe` | 无需 | 订阅 TODO 通知 |
| DELETE | `/api/v1/todos/:id/subscribe` | 无需 | 取消订阅 TODO 通知 |
| POST | `/api/v1/todos/:id/complete` | 需要 | 标记 TODO 为已完成 |

### 订阅者 (Subscribers)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| GET | `/api/v1/subscribers` | 无需 | 获取订阅者列表 |
| POST | `/api/v1/subscribers` | 无需 | 添加订阅者 |

### 导出 (Export)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| GET | `/api/v1/export/posts` | 需要 | 导出文章（JSON/CSV） |

### 备份 (Backup)

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| POST | `/api/v1/backup` | 需要 | 创建数据库备份 |
| GET | `/api/v1/backup/list` | 需要 | 获取备份列表 |
| POST | `/api/v1/backup/restore` | 需要 | 从备份恢复数据库 |

### 其他

| 方法 | 路径 | 认证 | 说明 |
|------|------|------|------|
| GET | `/api/health` | 无需 | 健康检查 |
| GET | `/api-docs/openapi.json` | 无需 | OpenAPI 规范 |

---

## 端点详情

---

### 认证 (Auth)

---

#### POST /api/v1/auth/register

注册新用户。密码使用 bcrypt 加密存储。

**认证**: 无需

**请求体**:
```json
{
  "email": "user@example.com",
  "password": "password123",
  "name": "用户名"
}
```

**响应 (201)**:
```json
{
  "user": {
    "id": 1,
    "email": "user@example.com",
    "name": "用户名",
    "role": "visitor",
    "avatar_url": null
  },
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000"
}
```

**错误**:
- 400: 邮箱格式无效、密码少于 6 位、名称为空
- 409: 邮箱已注册
- 500: 数据库不可用

---

#### POST /api/v1/auth/login

使用邮箱和密码登录。

**认证**: 无需

**请求体**:
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**响应 (200)**:
```json
{
  "user": {
    "id": 1,
    "email": "user@example.com",
    "name": "用户名",
    "role": "visitor",
    "avatar_url": null
  },
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000"
}
```

**错误**:
- 400: 邮箱或密码错误
- 500: 数据库不可用

---

#### POST /api/v1/auth/refresh

使用 refresh_token 获取新的 access_token。refresh_token 使用后即失效（一次性）。

**认证**: 无需

**请求体**:
```json
{
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000"
}
```

**响应 (200)**:
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

**错误**:
- 400: refresh_token 无效或已过期
- 404: 用户不存在

---

#### GET /api/v1/auth/me

获取当前认证用户的信息。支持 JWT token 和 ADMIN_KEY 两种认证方式。

**认证**: JWT Bearer Token 或 ADMIN_KEY

**请求头**:
```
Authorization: Bearer <token>
```

**响应 (200)**:
```json
{
  "user": {
    "id": 1,
    "email": "user@example.com",
    "name": "用户名",
    "role": "admin",
    "avatar_url": "https://avatars.githubusercontent.com/u/12345"
  }
}
```

**错误**:
- 400: token 无效或缺失
- 404: 用户不存在

---

#### GET /api/v1/auth/github

重定向浏览器到 GitHub OAuth 授权页面。需要配置 `GITHUB_CLIENT_ID` 环境变量。

**认证**: 无需

**响应**: 302 重定向到 GitHub

---

#### GET /api/v1/auth/github/callback

处理 GitHub OAuth 回调。交换授权码获取 GitHub access_token，获取用户资料，创建或关联账户，然后重定向到前端并携带 JWT token。

**认证**: 无需

**查询参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `code` | string | GitHub 授权码 |
| `state` | string | CSRF 防护状态码 |

**响应**: 302 重定向到 `{FRONTEND_URL}/auth/callback#access_token={jwt}&refresh_token={refresh}`

**账户关联逻辑**:
1. 按 `github_id` 匹配已有用户，直接登录
2. 按邮箱匹配已有用户，关联 GitHub 账户
3. 无匹配，创建新的 visitor 用户

---

#### GET /api/v1/auth/github/token

GitHub OAuth 的 JSON 版回调。返回 token 而非重定向，适合 API 客户端或测试。

**认证**: 无需

**查询参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `code` | string | GitHub 授权码 |

**响应 (200)**:
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000",
  "token_type": "Bearer"
}
```

**错误**:
- 400: 缺少 code 参数或 GitHub OAuth 错误
- 500: 数据库不可用

---

### 文章 (Posts)

---

#### GET /api/v1/posts

分页获取文章列表，按创建时间降序排列。

**认证**: 无需

**查询参数**:
| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `page` | u64 | 1 | 页码（最小 1） |
| `per_page` | u64 | 10 | 每页条数（1-100） |
| `status` | string | - | 按状态筛选：`published` 或 `draft` |

**响应 (200)**:
```json
{
  "items": [
    {
      "id": 1,
      "title": "我的第一篇文章",
      "slug": "my-first-post",
      "content": "文章内容...",
      "excerpt": "简短摘要",
      "cover_image": "https://example.com/cover.jpg",
      "status": "published",
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 50,
  "page": 1,
  "per_page": 10
}
```

---

#### POST /api/v1/posts

创建新文章。文章内容中的 `#todo` 标签会被自动提取为待办事项。

**认证**: 需要

**请求体**:
```json
{
  "title": "文章标题",
  "content": "文章正文。#todo 需要补充示例代码 #todo 修复排版问题",
  "excerpt": "可选的摘要",
  "cover_image": "https://example.com/cover.jpg",
  "status": "draft"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `title` | string | 是 | 文章标题（不可为空） |
| `content` | string | 是 | 文章内容 |
| `excerpt` | string | 否 | 摘要 |
| `cover_image` | string | 否 | 封面图 URL |
| `status` | string | 否 | `draft`（默认）或 `published` |

**响应 (201)**:
```json
{
  "id": 1,
  "title": "文章标题",
  "slug": "article-title",
  "content": "文章正文。#todo 需要补充示例代码 #todo 修复排版问题",
  "excerpt": "可选的摘要",
  "cover_image": "https://example.com/cover.jpg",
  "status": "draft",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z",
  "todos_created": [
    {
      "id": 1,
      "post_id": 1,
      "title": "需要补充示例代码",
      "description": null,
      "completed": false,
      "created_at": "2025-01-01T00:00:00Z"
    },
    {
      "id": 2,
      "post_id": 1,
      "title": "修复排版问题",
      "description": null,
      "completed": false,
      "created_at": "2025-01-01T00:00:00Z"
    }
  ]
}
```

**错误**:
- 400: 标题为空

---

#### GET /api/v1/posts/search

按标题和内容搜索文章。

**认证**: 无需

**查询参数**:
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `q` | string | 是 | 搜索关键词 |

**响应 (200)**:
```json
{
  "items": [
    {
      "id": 1,
      "title": "包含关键词的文章",
      "slug": "article-with-keyword",
      "content": "...",
      "excerpt": null,
      "cover_image": null,
      "status": "published",
      "created_at": "2025-01-01T00:00:00Z",
      "updated_at": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1,
  "page": 1,
  "per_page": 1
}
```

---

#### GET /api/v1/posts/:id

获取单篇文章详情。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**响应 (200)**:
```json
{
  "id": 1,
  "title": "文章标题",
  "slug": "article-title",
  "content": "完整文章内容...",
  "excerpt": "摘要",
  "cover_image": "https://example.com/cover.jpg",
  "status": "published",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-02T12:00:00Z"
}
```

**错误**:
- 404: 文章不存在

---

#### PUT /api/v1/posts/:id

更新文章。所有字段均可选，仅更新提供的字段。更新前会自动创建修订版本快照。如果文章从草稿变为已发布状态，会自动通知订阅者。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**请求体**:
```json
{
  "title": "更新后的标题",
  "content": "更新后的内容",
  "excerpt": "新的摘要",
  "cover_image": "https://example.com/new-cover.jpg",
  "status": "published"
}
```

**响应 (200)**: 返回更新后的 `PostResponse`

**错误**:
- 400: 标题为空
- 404: 文章不存在

---

#### DELETE /api/v1/posts/:id

删除文章及其关联的待办事项。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**响应**: 204 No Content

**错误**:
- 404: 文章不存在

---

#### GET /api/v1/posts/:id/todos

获取文章的所有待办事项（从 `#todo` 标签自动提取）。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**响应 (200)**:
```json
[
  {
    "id": 1,
    "post_id": 1,
    "title": "需要补充示例代码",
    "description": null,
    "completed": false,
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

**错误**:
- 404: 文章不存在

---

### 标签 (Tags)

---

#### GET /api/v1/tags

获取所有标签，按名称字母序排列。

**认证**: 无需

**响应 (200)**:
```json
[
  {
    "id": 1,
    "name": "Rust",
    "slug": "rust",
    "created_at": "2025-01-01T00:00:00Z"
  },
  {
    "id": 2,
    "name": "Vue",
    "slug": "vue",
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

---

#### POST /api/v1/tags

创建新标签。slug 从名称自动生成。

**认证**: 需要

**请求体**:
```json
{
  "name": "Rust"
}
```

**响应 (201)**:
```json
{
  "id": 1,
  "name": "Rust",
  "slug": "rust",
  "created_at": "2025-01-01T00:00:00Z"
}
```

**错误**:
- 400: 标签名为空
- 409: 标签名已存在

---

#### DELETE /api/v1/tags/:id

删除标签及其所有文章关联。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 标签 ID |

**响应**: 204 No Content

**错误**:
- 404: 标签不存在

---

#### GET /api/v1/posts/:id/tags

获取文章的所有标签。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**响应 (200)**:
```json
[
  {
    "id": 1,
    "name": "Rust",
    "slug": "rust",
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

**错误**:
- 404: 文章不存在

---

#### POST /api/v1/posts/:id/tags

为文章批量分配标签。已存在的关联会被跳过。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**请求体**:
```json
{
  "tag_ids": [1, 2, 3]
}
```

**响应 (200)**: 返回文章当前的所有标签
```json
[
  {
    "id": 1,
    "name": "Rust",
    "slug": "rust",
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

**错误**:
- 404: 文章或标签不存在

---

#### DELETE /api/v1/posts/:id/tags/:tag_id

移除文章与标签的关联。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |
| `tag_id` | i32 | 标签 ID |

**响应**: 204 No Content

**错误**:
- 404: 关联不存在

---

### 评论 (Comments)

---

#### POST /api/v1/posts/:id/comments

提交文章评论。评论默认为未审核状态，需要管理员审核后才会对外显示。

**认证**: 无需（公开接口）

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**请求体**:
```json
{
  "author_name": "读者昵称",
  "author_email": "reader@example.com",
  "content": "写得不错，学到了很多！"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `author_name` | string | 是 | 评论者昵称（不可为空） |
| `author_email` | string | 否 | 评论者邮箱 |
| `content` | string | 是 | 评论内容（不可为空） |

**响应 (201)**:
```json
{
  "id": 1,
  "post_id": 1,
  "author_name": "读者昵称",
  "author_email": "reader@example.com",
  "content": "写得不错，学到了很多！",
  "approved": false,
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

**错误**:
- 400: 昵称或内容为空
- 404: 文章不存在

---

#### GET /api/v1/posts/:id/comments

获取文章的已审核评论列表，按创建时间降序排列。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**响应 (200)**:
```json
[
  {
    "id": 1,
    "post_id": 1,
    "author_name": "读者昵称",
    "author_email": "reader@example.com",
    "content": "写得不错！",
    "approved": true,
    "created_at": "2025-01-01T00:00:00Z",
    "updated_at": "2025-01-01T00:00:00Z"
  }
]
```

**错误**:
- 404: 文章不存在

---

#### GET /api/v1/comments/pending

获取所有文章中待审核的评论列表，按创建时间降序排列。

**认证**: 需要

**响应 (200)**:
```json
[
  {
    "id": 2,
    "post_id": 1,
    "author_name": "新读者",
    "author_email": null,
    "content": "请问这个怎么用？",
    "approved": false,
    "created_at": "2025-01-02T00:00:00Z",
    "updated_at": "2025-01-02T00:00:00Z"
  }
]
```

---

#### PUT /api/v1/comments/:id/approve

审核通过评论，将 `approved` 设置为 `true`。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 评论 ID |

**响应 (200)**: 返回更新后的 `CommentResponse`（`approved` 为 `true`）

**错误**:
- 404: 评论不存在

---

#### DELETE /api/v1/comments/:id

删除评论。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 评论 ID |

**响应**: 204 No Content

**错误**:
- 404: 评论不存在

---

### 版本控制 (Revisions)

---

#### POST /api/v1/posts/:id/revisions

为文章创建修订版本快照，保存当前标题、内容、摘要和封面图。同时更新文章的 `current_version`。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**响应 (201)**:
```json
{
  "id": 1,
  "post_id": 1,
  "title": "文章标题",
  "content": "当时的完整内容...",
  "excerpt": "当时的摘要",
  "cover_image": "https://...",
  "version": 1,
  "created_by": null,
  "created_at": "2025-01-01T00:00:00Z"
}
```

**错误**:
- 404: 文章不存在

---

#### GET /api/v1/posts/:id/revisions

获取文章的所有修订版本，按版本号降序排列。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**响应 (200)**:
```json
{
  "items": [
    {
      "id": 2,
      "post_id": 1,
      "title": "文章标题",
      "content": "第二次快照内容...",
      "excerpt": null,
      "cover_image": null,
      "version": 2,
      "created_by": null,
      "created_at": "2025-01-02T00:00:00Z"
    },
    {
      "id": 1,
      "post_id": 1,
      "title": "文章标题",
      "content": "第一次快照内容...",
      "excerpt": null,
      "cover_image": null,
      "version": 1,
      "created_by": null,
      "created_at": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 2
}
```

**错误**:
- 404: 文章不存在

---

#### GET /api/v1/posts/:id/revisions/:rev_id

获取特定修订版本的详情。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |
| `rev_id` | i32 | 修订版本 ID |

**响应 (200)**: 返回 `RevisionResponse`

**错误**:
- 404: 修订版本不存在

---

#### POST /api/v1/posts/:id/revisions/:rev_id/rollback

回滚文章到指定修订版本。会将文章内容恢复为目标版本的状态，并创建新的修订版本记录。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |
| `rev_id` | i32 | 目标修订版本 ID |

**响应 (200)**: 返回新创建的 `RevisionResponse`（包含回滚后的内容）

**错误**:
- 404: 文章或修订版本不存在

---

#### GET /api/v1/posts/:id/diff

比较两个修订版本之间的内容差异，生成 unified diff 格式。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**查询参数**:
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `from` | i32 | 是 | 源修订版本 ID |
| `to` | i32 | 是 | 目标修订版本 ID |

**响应 (200)**:
```json
{
  "diff": " 这是相同的内容\n-这是旧的内容\n+这是新的内容\n  这也是相同的内容",
  "from_version": 1,
  "to_version": 2
}
```

**错误**:
- 404: 文章或修订版本不存在

---

### PR (Pull Requests)

---

#### POST /api/v1/posts/:id/pulls

为文章创建 PR（拉取请求）。PR 基于片段（Fragment）机制，允许精确指定要修改的文本位置。

**认证**: 无需（公开接口）

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**请求体**:
```json
{
  "user_email": "contributor@example.com",
  "fragments": [
    {
      "start_line": 5,
      "start_col": 0,
      "end_line": 5,
      "end_col": 20,
      "replacement": "修正后的文本",
      "description": "修复拼写错误"
    }
  ],
  "message": "修复了第 5 行的拼写错误"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `user_email` | string | 是 | 提交者邮箱 |
| `fragments` | Fragment[] | 是 | 至少一个片段 |
| `message` | string | 否 | PR 描述信息 |

**Fragment 结构**:
| 字段 | 类型 | 说明 |
|------|------|------|
| `start_line` | i32 | 起始行号（从 1 开始） |
| `start_col` | i32 | 起始列号 |
| `end_line` | i32 | 结束行号 |
| `end_col` | i32 | 结束列号 |
| `replacement` | string | 替换文本 |
| `description` | string | 片段描述（可选） |

**响应 (201)**:
```json
{
  "id": 1,
  "post_id": 1,
  "user_email": "contributor@example.com",
  "content": "[0] 修复拼写错误",
  "status": "open",
  "fragments": [
    {
      "start_line": 5,
      "start_col": 0,
      "end_line": 5,
      "end_col": 20,
      "replacement": "修正后的文本",
      "description": "修复拼写错误"
    }
  ],
  "message": "修复了第 5 行的拼写错误",
  "created_at": "2025-01-01T00:00:00Z"
}
```

**错误**:
- 400: 片段为空、位置越界、片段重叠
- 404: 文章不存在

---

#### GET /api/v1/posts/:id/pulls

获取文章的所有 PR，按创建时间降序排列。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |

**响应 (200)**:
```json
{
  "items": [
    {
      "id": 1,
      "post_id": 1,
      "user_email": "contributor@example.com",
      "content": "[0] 修复拼写错误",
      "status": "open",
      "fragments": [...],
      "message": "修复了拼写错误",
      "created_at": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1
}
```

**错误**:
- 404: 文章不存在

---

#### PUT /api/v1/pulls/:id

更新 PR 状态。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | PR ID |

**请求体**:
```json
{
  "status": "closed"
}
```

状态值：`open`、`closed`、`merged`

**响应 (200)**: 返回更新后的 `PullResponse`

**错误**:
- 400: 状态值无效
- 404: PR 不存在

---

#### POST /api/v1/pulls/:id/apply

合并 PR：将片段应用到文章内容中。PR 必须处于 `open` 状态。合并后自动创建修订版本快照，PR 状态变为 `merged`。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | PR ID |

**响应 (200)**:
```json
{
  "pull_request": {
    "id": 1,
    "post_id": 1,
    "user_email": "contributor@example.com",
    "content": "[0] 修复拼写错误",
    "status": "merged",
    "fragments": [...],
    "message": "修复了拼写错误",
    "created_at": "2025-01-01T00:00:00Z"
  },
  "updated_post_id": 1,
  "revision_version": 3
}
```

**错误**:
- 400: PR 不是 open 状态、没有片段、片段应用失败
- 404: PR 或文章不存在

---

#### POST /api/v1/pulls/:id/comments

为 PR 添加行内评论，可精确到某个片段的某一行。

**认证**: 无需（公开接口）

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | PR ID |

**请求体**:
```json
{
  "fragment_index": 0,
  "line": 5,
  "content": "同意这个修改",
  "user_email": "reader@example.com"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `fragment_index` | i32 | 是 | 片段索引（从 0 开始） |
| `line` | i32 | 是 | 行号 |
| `content` | string | 是 | 评论内容（不可为空） |
| `user_email` | string | 是 | 评论者邮箱 |

**响应 (201)**:
```json
{
  "id": 1,
  "pull_request_id": 1,
  "fragment_index": 0,
  "line": 5,
  "content": "同意这个修改",
  "user_email": "reader@example.com",
  "created_at": "2025-01-01T00:00:00Z"
}
```

**错误**:
- 400: 评论内容为空
- 404: PR 不存在

---

#### GET /api/v1/pulls/:id/comments

获取 PR 的所有行内评论，按创建时间升序排列。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | PR ID |

**响应 (200)**:
```json
{
  "items": [
    {
      "id": 1,
      "pull_request_id": 1,
      "fragment_index": 0,
      "line": 5,
      "content": "同意这个修改",
      "user_email": null,
      "created_at": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1
}
```

**错误**:
- 404: PR 不存在

---

### 媒体 (Media)

---

#### POST /api/v1/media

上传媒体文件（图片或视频）。文件存储在 `static/media/` 目录，元数据保存到数据库。

**认证**: 需要

**请求格式**: `multipart/form-data`

**请求字段**:
| 字段 | 类型 | 说明 |
|------|------|------|
| `file` | file | 文件内容 |

**支持的格式**:
- 图片：JPEG、PNG、GIF、WebP
- 视频：MP4、WebM

**文件大小限制**: 100MB

**响应 (201)**:
```json
{
  "id": 1,
  "filename": "550e8400-e29b-41d4-a716-446655440000.jpg",
  "original_name": "photo.jpg",
  "mime_type": "image/jpeg",
  "size_bytes": 1024000,
  "url": "/static/media/550e8400-e29b-41d4-a716-446655440000.jpg",
  "created_at": "2025-01-01T00:00:00Z"
}
```

**错误**:
- 400: 缺少 file 字段、格式不支持
- 413: 文件过大

---

#### GET /api/v1/media

分页获取媒体文件列表，按创建时间降序排列。

**认证**: 无需

**查询参数**:
| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `page` | u64 | 1 | 页码 |
| `per_page` | u64 | 20 | 每页条数（1-100） |

**响应 (200)**:
```json
{
  "items": [
    {
      "id": 1,
      "filename": "550e8400-e29b-41d4-a716-446655440000.jpg",
      "original_name": "photo.jpg",
      "mime_type": "image/jpeg",
      "size_bytes": 1024000,
      "url": "/static/media/550e8400-e29b-41d4-a716-446655440000.jpg",
      "created_at": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 50,
  "page": 1,
  "per_page": 20
}
```

---

#### DELETE /api/v1/media/:id

删除媒体文件（同时删除文件和数据库记录）。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 媒体文件 ID |

**响应**: 204 No Content

**错误**:
- 404: 媒体文件不存在

---

### 图片 (Images)

---

#### POST /api/v1/images

上传图片文件。这是图片专用上传接口，文件存储在 `static/uploads/` 目录。

**认证**: 需要

**请求格式**: `multipart/form-data`

**请求字段**:
| 字段 | 类型 | 说明 |
|------|------|------|
| `file` | file | 图片文件 |

**支持的格式**: JPEG、PNG、GIF、WebP

**文件大小限制**: 5MB

**响应 (200)**:
```json
{
  "url": "/static/uploads/550e8400-e29b-41d4-a716-446655440000.jpg",
  "filename": "550e8400-e29b-41d4-a716-446655440000.jpg"
}
```

**错误**:
- 400: 缺少 file 字段、格式不支持
- 413: 文件过大

---

### 视频 (Videos)

---

#### POST /api/v1/videos

上传视频文件。文件存储在 `static/videos/` 目录。

**认证**: 需要

**请求格式**: `multipart/form-data`

**请求字段**:
| 字段 | 类型 | 说明 |
|------|------|------|
| `file` | file | 视频文件 |

**支持的格式**: MP4、WebM、OGG

**文件大小限制**: 100MB

**响应 (200)**:
```json
{
  "url": "/static/videos/550e8400-e29b-41d4-a716-446655440000.mp4",
  "filename": "550e8400-e29b-41d4-a716-446655440000.mp4"
}
```

**错误**:
- 400: 缺少 file 字段、格式不支持
- 413: 文件过大

---

### TODO

---

#### GET /api/v1/todos

获取 TODO 列表。管理员获取所有 TODO，普通用户仅获取已发布文章中的 TODO。

**认证**: 无需（但 ADMIN_KEY 可获取更多数据）

**响应 (200)**:
```json
{
  "items": [
    {
      "id": 1,
      "post_id": 1,
      "title": "需要补充示例代码",
      "completed": false,
      "subscriber_count": 3
    },
    {
      "id": 2,
      "post_id": 1,
      "title": "修复排版问题",
      "completed": true,
      "subscriber_count": 1
    }
  ]
}
```

---

#### POST /api/v1/todos/:id/subscribe

订阅 TODO 通知。当 TODO 被标记为完成时，订阅者会收到邮件通知。需要提供 `email` 或 `user_id` 之一。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | TODO ID |

**请求体**:
```json
{
  "email": "user@example.com"
}
```

或

```json
{
  "user_id": 1
}
```

**响应**: 201 Created

**错误**:
- 400: 未提供 email 或 user_id
- 404: TODO 不存在
- 409: 已订阅

---

#### DELETE /api/v1/todos/:id/subscribe

取消订阅 TODO 通知。

**认证**: 无需

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | TODO ID |

**请求体**:
```json
{
  "email": "user@example.com"
}
```

**响应**: 204 No Content

**错误**:
- 400: 未提供 email 或 user_id
- 404: 订阅不存在

---

#### POST /api/v1/todos/:id/complete

标记 TODO 为已完成，并向所有订阅者发送邮件通知。

**认证**: 需要

**路径参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | TODO ID |

**响应 (200)**:
```json
{
  "id": 1,
  "post_id": 1,
  "title": "需要补充示例代码",
  "completed": true,
  "subscriber_count": 3
}
```

**错误**:
- 404: TODO 不存在

---

### 订阅者 (Subscribers)

---

#### GET /api/v1/subscribers

获取所有订阅者列表。

**认证**: 无需

**响应 (200)**:
```json
[
  {
    "id": 1,
    "email": "subscriber@example.com",
    "name": "订阅者名称",
    "confirmed": false
  }
]
```

---

#### POST /api/v1/subscribers

添加新的邮件订阅者。

**认证**: 无需

**请求体**:
```json
{
  "email": "newsubscriber@example.com",
  "name": "可选名称"
}
```

**响应 (201)**:
```json
{
  "id": 1,
  "email": "newsubscriber@example.com",
  "name": "可选名称",
  "confirmed": false
}
```

**错误**:
- 409: 邮箱已订阅

---

### 导出 (Export)

---

#### GET /api/v1/export/posts

导出所有文章为 JSON 或 CSV 格式。

**认证**: 需要

**查询参数**:
| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `format` | string | json | 导出格式：`json` 或 `csv` |
| `status` | string | - | 按状态筛选：`published` 或 `draft` |

**响应 (200)**:

JSON 格式（`Content-Type: application/json`）:
```json
[
  {
    "id": 1,
    "title": "文章标题",
    "slug": "article-title",
    "content": "完整内容...",
    "excerpt": "摘要",
    "cover_image": "https://...",
    "status": "published",
    "created_at": "2025-01-01T00:00:00Z",
    "updated_at": "2025-01-01T00:00:00Z"
  }
]
```

CSV 格式（`Content-Type: text/csv`）:
```csv
id,title,slug,content,excerpt,cover_image,status,created_at,updated_at
1,文章标题,article-title,完整内容...,摘要,https://...,published,2025-01-01T00:00:00Z,2025-01-01T00:00:00Z
```

---

### 备份 (Backup)

---

#### POST /api/v1/backup

创建数据库备份。使用 `pg_dump` 生成 SQL 备份文件。

**认证**: 需要

**响应 (200)**:
```json
{
  "message": "Backup created successfully",
  "filename": "backup_20250101_120000.sql",
  "size_bytes": 1024000
}
```

**错误**:
- 500: 备份失败

---

#### GET /api/v1/backup/list

获取所有备份文件列表。

**认证**: 需要

**响应 (200)**:
```json
{
  "backups": [
    {
      "filename": "backup_20250101_120000.sql",
      "size_bytes": 1024000,
      "created_at": "2025-01-01T12:00:00Z"
    },
    {
      "filename": "backup_20241231_120000.sql",
      "size_bytes": 980000,
      "created_at": "2024-12-31T12:00:00Z"
    }
  ],
  "total": 2
}
```

---

#### POST /api/v1/backup/restore

从备份文件恢复数据库。文件名不允许包含 `..`、`/` 或 `\`。

**认证**: 需要

**请求体**:
```json
{
  "filename": "backup_20250101_120000.sql"
}
```

**响应 (200)**:
```json
{
  "message": "Database restored successfully",
  "filename": "backup_20250101_120000.sql"
}
```

**错误**:
- 400: 文件名无效
- 404: 备份文件不存在
- 500: 恢复失败

---

### 其他

---

#### GET /api/health

健康检查端点。

**认证**: 无需

**响应 (200)**:
```json
{
  "status": "ok"
}
```

---

#### GET /api-docs/openapi.json

获取 OpenAPI 3.0 规范（JSON 格式）。

**认证**: 无需

**响应 (200)**: OpenAPI JSON 规范

---

## 数据模型

### PostResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 文章 ID |
| `title` | string | 标题 |
| `slug` | string | URL 友好的标识符（自动生成） |
| `content` | string | 完整内容 |
| `excerpt` | string? | 摘要 |
| `cover_image` | string? | 封面图 URL |
| `status` | string | `published` 或 `draft` |
| `created_at` | string | 创建时间（RFC 3339） |
| `updated_at` | string | 更新时间（RFC 3339） |

### TagResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 标签 ID |
| `name` | string | 标签名称 |
| `slug` | string | URL 友好的标识符 |
| `created_at` | string | 创建时间 |

### CommentResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 评论 ID |
| `post_id` | i32 | 文章 ID |
| `author_name` | string | 评论者昵称 |
| `author_email` | string? | 评论者邮箱 |
| `content` | string | 评论内容 |
| `approved` | bool | 是否已审核 |
| `created_at` | string | 创建时间 |
| `updated_at` | string | 更新时间 |

### RevisionResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 修订版本 ID |
| `post_id` | i32 | 文章 ID |
| `title` | string? | 快照时的标题 |
| `content` | string? | 快照时的内容 |
| `excerpt` | string? | 快照时的摘要 |
| `cover_image` | string? | 快照时的封面图 |
| `version` | i32 | 版本号 |
| `created_by` | i32? | 创建者用户 ID |
| `created_at` | string | 创建时间 |

### PullResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | PR ID |
| `post_id` | i32 | 文章 ID |
| `user_email` | string | 提交者邮箱 |
| `content` | string | 内容摘要 |
| `status` | string | `open`、`closed` 或 `merged` |
| `fragments` | json? | 片段列表 |
| `message` | string? | PR 描述 |
| `created_at` | string | 创建时间 |

### Fragment

| 字段 | 类型 | 说明 |
|------|------|------|
| `start_line` | i32 | 起始行号（从 1 开始） |
| `start_col` | i32 | 起始列号 |
| `end_line` | i32 | 结束行号 |
| `end_col` | i32 | 结束列号 |
| `replacement` | string | 替换文本 |
| `description` | string? | 片段描述 |

### MediaResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 媒体文件 ID |
| `filename` | string | 存储文件名（UUID） |
| `original_name` | string | 原始文件名 |
| `mime_type` | string | MIME 类型 |
| `size_bytes` | i64 | 文件大小（字节） |
| `url` | string | 访问 URL |
| `created_at` | string | 上传时间 |

### TodoResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | TODO ID |
| `post_id` | i32 | 所属文章 ID |
| `title` | string | TODO 标题 |
| `completed` | bool | 是否已完成 |
| `subscriber_count` | i64 | 订阅者数量 |

### SubscriberResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 订阅者 ID |
| `email` | string | 邮箱 |
| `name` | string? | 名称 |
| `confirmed` | bool | 是否已确认 |

### UserResponse

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | i32 | 用户 ID |
| `email` | string | 邮箱 |
| `name` | string | 用户名 |
| `role` | string | 角色 |
| `avatar_url` | string? | 头像 URL |

---

## 使用示例

### 创建并发布一篇文章

```bash
# 1. 创建草稿
curl -X POST http://localhost:8080/api/v1/posts \
  -H "Authorization: Bearer YOUR_ADMIN_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "使用 Rust 构建 Web API",
    "content": "本文介绍如何使用 Axum 框架...\n#todo 添加性能对比数据\n#todo 补充错误处理示例",
    "status": "draft"
  }'

# 2. 发布文章（ID 为 1）
curl -X PUT http://localhost:8080/api/v1/posts/1 \
  -H "Authorization: Bearer YOUR_ADMIN_KEY" \
  -H "Content-Type: application/json" \
  -d '{"status": "published"}'
```

### 提交 PR 修改文章

```bash
# 创建 PR
curl -X POST http://localhost:8080/api/v1/posts/1/pulls \
  -H "Content-Type: application/json" \
  -d '{
    "user_email": "reader@example.com",
    "fragments": [
      {
        "start_line": 3,
        "start_col": 0,
        "end_line": 3,
        "end_col": 10,
        "replacement": "Axum 是一个",
        "description": "补充主语"
      }
    ],
    "message": "建议补充主语使句子更完整"
  }'

# 管理员合并 PR
curl -X POST http://localhost:8080/api/v1/pulls/1/apply \
  -H "Authorization: Bearer YOUR_ADMIN_KEY"
```

### 评论审核流程

```bash
# 1. 读者提交评论
curl -X POST http://localhost:8080/api/v1/posts/1/comments \
  -H "Content-Type: application/json" \
  -d '{
    "author_name": "读者",
    "content": "写得很好！"
  }'

# 2. 管理员查看待审核评论
curl http://localhost:8080/api/v1/comments/pending \
  -H "Authorization: Bearer YOUR_ADMIN_KEY"

# 3. 管理员审核通过
curl -X PUT http://localhost:8080/api/v1/comments/1/approve \
  -H "Authorization: Bearer YOUR_ADMIN_KEY"
```

### 版本管理

```bash
# 查看修订历史
curl http://localhost:8080/api/v1/posts/1/revisions

# 比较两个版本的差异
curl "http://localhost:8080/api/v1/posts/1/diff?from=1&to=2"

# 回滚到版本 1
curl -X POST http://localhost:8080/api/v1/posts/1/revisions/1/rollback \
  -H "Authorization: Bearer YOUR_ADMIN_KEY"
```

### 备份与恢复

```bash
# 创建备份
curl -X POST http://localhost:8080/api/v1/backup \
  -H "Authorization: Bearer YOUR_ADMIN_KEY"

# 查看备份列表
curl http://localhost:8080/api/v1/backup/list \
  -H "Authorization: Bearer YOUR_ADMIN_KEY"

# 恢复备份
curl -X POST http://localhost:8080/api/v1/backup/restore \
  -H "Authorization: Bearer YOUR_ADMIN_KEY" \
  -H "Content-Type: application/json" \
  -d '{"filename": "backup_20250101_120000.sql"}'
```
