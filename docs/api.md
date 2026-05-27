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

### PR 式互动

#### 获取文章的 PR 列表

```
GET /api/v1/posts/:id/pulls
```

响应：`PullListResponse`，包含 `items` 和 `total`

#### 创建 PR

```
POST /api/v1/posts/:id/pulls
Content-Type: application/json

{
  "user_email": "reader@example.com",
  "content": "建议修改：这里应该用 async/await"
}
```

响应：201 返回 `PullResponse`

#### 更新 PR 状态

```
PUT /api/v1/pulls/:id
Content-Type: application/json

{
  "status": "merged"
}
```

状态值：`open`、`closed`、`merged`

响应：`PullResponse`

#### 添加评论

```
POST /api/v1/pulls/:id/comments
Content-Type: application/json

{
  "user_email": "reader@example.com",
  "content": "同意这个修改，已合并"
}
```

响应：201 返回 `CommentResponse`

---

### 数据备份

#### 创建备份（需要认证）

```
POST /api/v1/backup
Authorization: Bearer YOUR_ADMIN_KEY
```

响应：`BackupResponse`，包含 `message`、`filename`、`size_bytes`

#### 获取备份列表（需要认证）

```
GET /api/v1/backup/list
Authorization: Bearer YOUR_ADMIN_KEY
```

响应：`BackupListResponse`，包含 `backups` 数组和 `total`

#### 恢复备份（需要认证）

```
POST /api/v1/backup/restore
Content-Type: application/json
Authorization: Bearer YOUR_ADMIN_KEY

{
  "filename": "backup_20250101_120000.sql"
}
```

响应：`RestoreResponse`

---

### 视频上传

#### 上传视频（需要认证）

```
POST /api/v1/videos
Content-Type: multipart/form-data
Authorization: Bearer YOUR_ADMIN_KEY

file: <视频文件>
```

支持格式：MP4、WebM、OGG
最大文件大小：100MB

响应：
```json
{
  "url": "/static/videos/550e8400-e29b-41d4-a716-446655440000.mp4",
  "filename": "550e8400-e29b-41d4-a716-446655440000.mp4"
}
```

---

### 静态文件

```
GET /static/:filename
```

提供 `static/` 目录下的上传图片和视频。

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

### PullResponse（PR 响应）

```json
{
  "id": 1,
  "post_id": 1,
  "user_email": "reader@example.com",
  "content": "建议修改：这里应该用 async/await",
  "status": "open",
  "created_at": "2025-01-01T00:00:00Z"
}
```

### PullListResponse（PR 列表响应）

```json
{
  "items": [
    {
      "id": 1,
      "post_id": 1,
      "user_email": "reader@example.com",
      "content": "建议修改",
      "status": "open",
      "created_at": "2025-01-01T00:00:00Z"
    }
  ],
  "total": 1
}
```

### CommentResponse（评论响应）

```json
{
  "id": 1,
  "pull_request_id": 1,
  "user_email": "reader@example.com",
  "content": "同意这个修改",
  "created_at": "2025-01-01T00:00:00Z"
}
```

### BackupResponse（备份响应）

```json
{
  "message": "Backup created successfully",
  "filename": "backup_20250101_120000.sql",
  "size_bytes": 1024000
}
```

### BackupListResponse（备份列表响应）

```json
{
  "backups": [
    {
      "filename": "backup_20250101_120000.sql",
      "size_bytes": 1024000,
      "created_at": "2025-01-01T12:00:00Z"
    }
  ],
  "total": 1
}
```

### RestoreResponse（恢复响应）

```json
{
  "message": "Database restored successfully",
  "filename": "backup_20250101_120000.sql"
}
```
