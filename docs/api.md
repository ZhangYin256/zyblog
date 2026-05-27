# API 参考文档

基础 URL: `http://localhost:8080`

交互式文档: `http://localhost:8080/swagger-ui/`

## 认证

管理员接口需要 Bearer 令牌：

```
Authorization: Bearer YOUR_ADMIN_KEY
```

只读的 GET 接口是公开的。

## 接口端点

### 健康检查

```
GET /api/health
```

响应: `{ "status": "ok" }`

---

### 文章

#### 获取文章列表

```
GET /api/v1/posts?page=1&per_page=10&status=published
```

查询参数：
- `page`（可选）：页码，默认 1
- `per_page`（可选）：每页条数，默认 10，最大 100
- `status`（可选）：按 "published"（已发布）或 "draft"（草稿）筛选

响应：`PostListResponse`，包含 `items`、`total`、`page`、`per_page`

#### 获取单篇文章

```
GET /api/v1/posts/:id
```

响应：`PostResponse`

#### 创建文章（需要认证）

```
POST /api/v1/posts
Content-Type: application/json
Authorization: Bearer YOUR_ADMIN_KEY

{
  "title": "我的文章",
  "content": "文章内容，包含 #todo 任务",
  "excerpt": "简短摘要",
  "cover_image": "https://...",
  "status": "published"
}
```

响应：`CreatePostResponse`（文章 + 创建的待办事项）

#### 更新文章（需要认证）

```
PUT /api/v1/posts/:id
Content-Type: application/json
Authorization: Bearer YOUR_ADMIN_KEY

{
  "title": "更新后的标题",
  "content": "更新后的内容",
  "status": "published"
}
```

所有字段均为可选。仅更新提供的字段。

#### 删除文章（需要认证）

```
DELETE /api/v1/posts/:id
Authorization: Bearer YOUR_ADMIN_KEY
```

响应：204 No Content

#### 获取文章待办事项

```
GET /api/v1/posts/:id/todos
```

响应：`TodoItemResponse` 数组

---

### 订阅者

#### 获取订阅者列表

```
GET /api/v1/subscribers
```

响应：`SubscriberResponse` 数组

#### 创建订阅者

```
POST /api/v1/subscribers
Content-Type: application/json

{
  "email": "user@example.com",
  "name": "可选名称"
}
```

响应：201 返回 `SubscriberResponse`，若邮箱已存在则返回 409

---

### 导出

#### 导出文章（需要认证）

```
GET /api/v1/export/posts?format=json&status=published
Authorization: Bearer YOUR_ADMIN_KEY
```

查询参数：
- `format`（可选）："json"（默认）或 "csv"
- `status`（可选）：按 "published"（已发布）或 "draft"（草稿）筛选

响应：JSON 数组或 CSV 文件下载

---

### 静态文件

```
GET /static/:filename
```

提供 `static/` 目录下的上传图片。

## 数据类型

### PostResponse（文章响应）

```json
{
  "id": 1,
  "title": "我的文章",
  "slug": "my-post",
  "content": "完整内容...",
  "excerpt": "简短摘要",
  "cover_image": "https://...",
  "status": "published",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

### TodoItemResponse（待办事项响应）

```json
{
  "id": 1,
  "post_id": 1,
  "title": "修复这个 bug",
  "description": null,
  "completed": false,
  "created_at": "2025-01-01T00:00:00Z"
}
```

### SubscriberResponse（订阅者响应）

```json
{
  "id": 1,
  "email": "user@example.com",
  "name": "可选名称",
  "confirmed": false
}
```

### ExportedPost（导出文章）

与 `PostResponse` 字段相同。用于 JSON/CSV 导出。
