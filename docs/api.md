# API Reference

Base URL: `http://localhost:8080`

Interactive docs: `http://localhost:8080/swagger-ui/`

## Authentication

Admin endpoints require a Bearer token:

```
Authorization: Bearer YOUR_ADMIN_KEY
```

Read-only GET endpoints are public.

## Endpoints

### Health Check

```
GET /api/health
```

Response: `{ "status": "ok" }`

---

### Posts

#### List Posts

```
GET /api/v1/posts?page=1&per_page=10&status=published
```

Query parameters:
- `page` (optional): Page number, default 1
- `per_page` (optional): Items per page, default 10, max 100
- `status` (optional): Filter by "published" or "draft"

Response: `PostListResponse` with `items`, `total`, `page`, `per_page`

#### Get Post

```
GET /api/v1/posts/:id
```

Response: `PostResponse`

#### Create Post (auth required)

```
POST /api/v1/posts
Content-Type: application/json
Authorization: Bearer YOUR_ADMIN_KEY

{
  "title": "My Post",
  "content": "Post content with #todo tasks",
  "excerpt": "Short summary",
  "cover_image": "https://...",
  "status": "published"
}
```

Response: `CreatePostResponse` (post + created todos)

#### Update Post (auth required)

```
PUT /api/v1/posts/:id
Content-Type: application/json
Authorization: Bearer YOUR_ADMIN_KEY

{
  "title": "Updated Title",
  "content": "Updated content",
  "status": "published"
}
```

All fields are optional. Only provided fields are updated.

#### Delete Post (auth required)

```
DELETE /api/v1/posts/:id
Authorization: Bearer YOUR_ADMIN_KEY
```

Response: 204 No Content

#### Get Post Todos

```
GET /api/v1/posts/:id/todos
```

Response: Array of `TodoItemResponse`

---

### Subscribers

#### List Subscribers

```
GET /api/v1/subscribers
```

Response: Array of `SubscriberResponse`

#### Create Subscriber

```
POST /api/v1/subscribers
Content-Type: application/json

{
  "email": "user@example.com",
  "name": "Optional Name"
}
```

Response: 201 with `SubscriberResponse`, or 409 if email exists

---

### Export

#### Export Posts (auth required)

```
GET /api/v1/export/posts?format=json&status=published
Authorization: Bearer YOUR_ADMIN_KEY
```

Query parameters:
- `format` (optional): "json" (default) or "csv"
- `status` (optional): Filter by "published" or "draft"

Response: JSON array or CSV file download

---

### Static Files

```
GET /static/:filename
```

Serves uploaded images from the `static/` directory.

## Data Types

### PostResponse

```json
{
  "id": 1,
  "title": "My Post",
  "slug": "my-post",
  "content": "Full content...",
  "excerpt": "Short summary",
  "cover_image": "https://...",
  "status": "published",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

### TodoItemResponse

```json
{
  "id": 1,
  "post_id": 1,
  "title": "Fix the bug",
  "description": null,
  "completed": false,
  "created_at": "2025-01-01T00:00:00Z"
}
```

### SubscriberResponse

```json
{
  "id": 1,
  "email": "user@example.com",
  "name": "Optional Name",
  "confirmed": false
}
```

### ExportedPost

Same fields as `PostResponse`. Used for JSON/CSV export.
