# ZYBlog

A modern blog platform built with Vue 3 + Axum (Rust) + PostgreSQL + SeaORM.

## Tech Stack

- **Frontend**: Vue 3 + Vite
- **Backend**: Axum (Rust)
- **Database**: PostgreSQL 15
- **ORM**: SeaORM
- **API Docs**: utoipa (OpenAPI 3.0) + Swagger UI

## Quick Start

### Prerequisites

- Docker and Docker Compose
- (Optional) Rust toolchain for local development

### Using Docker

```bash
# Clone the repo
git clone <repo-url>
cd zyblog

# Copy environment file
cp .env.example .env

# Edit .env to set your ADMIN_KEY and other settings
# vim .env

# Start all services
docker-compose up -d

# Run database migrations
make migrate
```

The application will be available at:

- **Frontend**: http://localhost:5173
- **Backend API**: http://localhost:8080
- **Swagger UI**: http://localhost:8080/swagger-ui/

### Local Development

```bash
# Start only the database
docker-compose up -d postgres

# Set environment variables
export DATABASE_URL=postgres://zyblog:zyblog_dev@localhost:5432/zyblog
export ADMIN_KEY=your_admin_key

# Run backend
cd backend
cargo run

# Run frontend (in another terminal)
cd frontend
npm install
npm run dev
```

## Environment Variables

| Variable | Description | Default |
|---|---|---|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://zyblog:zyblog_dev@postgres:5432/zyblog` |
| `ADMIN_KEY` | Bearer token for admin API access | (required) |
| `SERVER_ADDR` | Backend listen address | `0.0.0.0:8080` |
| `RUST_LOG` | Log level | `zyblog=debug,tower_http=debug` |
| `SMTP_HOST` | SMTP server for email notifications | (empty) |
| `SMTP_PORT` | SMTP port | `587` |
| `SMTP_USERNAME` | SMTP username | (empty) |
| `SMTP_PASSWORD` | SMTP password | (empty) |
| `SMTP_FROM` | Sender email address | `noreply@zyblog.local` |
| `VITE_API_BASE_URL` | Frontend API base URL | `http://localhost:8080` |

## API Overview

All endpoints are prefixed with `/api/v1`.

### Posts

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/posts` | List posts (paginated) |
| `POST` | `/api/v1/posts` | Create a post (auth required) |
| `GET` | `/api/v1/posts/:id` | Get a single post |
| `PUT` | `/api/v1/posts/:id` | Update a post (auth required) |
| `DELETE` | `/api/v1/posts/:id` | Delete a post (auth required) |
| `GET` | `/api/v1/posts/:id/todos` | Get todo items for a post |

### Subscribers

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/subscribers` | List subscribers |
| `POST` | `/api/v1/subscribers` | Add a subscriber |

### Export

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/export/posts` | Export posts as JSON or CSV (auth required) |

### Other

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/health` | Health check |
| `GET` | `/swagger-ui/` | Swagger UI |
| `GET` | `/api-docs/openapi.json` | OpenAPI spec |

Full interactive documentation is available at `/swagger-ui/` when the server is running.

## Authentication

Write operations (POST, PUT, DELETE) require a Bearer token in the `Authorization` header:

```bash
curl -H "Authorization: Bearer YOUR_ADMIN_KEY" \
     -X POST http://localhost:8080/api/v1/posts \
     -H "Content-Type: application/json" \
     -d '{"title": "Hello", "content": "World"}'
```

Read operations (GET) are public and do not require authentication.

## Data Export

Export all blog posts in JSON or CSV format:

```bash
# JSON export
curl -H "Authorization: Bearer YOUR_ADMIN_KEY" \
     http://localhost:8080/api/v1/export/posts \
     -o posts.json

# CSV export
curl -H "Authorization: Bearer YOUR_ADMIN_KEY" \
     "http://localhost:8080/api/v1/export/posts?format=csv" \
     -o posts.csv

# Filter by status
curl -H "Authorization: Bearer YOUR_ADMIN_KEY" \
     "http://localhost:8080/api/v1/export/posts?status=published" \
     -o published.json
```

## Makefile Commands

| Command | Description |
|---|---|
| `make up` | Start all services |
| `make down` | Stop all services |
| `make logs` | Follow all logs |
| `make logs-backend` | Follow backend logs |
| `make build` | Build all services |
| `make test` | Run backend tests |
| `make migrate` | Run database migrations |
| `make db-shell` | Open psql shell |
| `make clean` | Remove volumes and containers |

## Project Structure

```
zyblog/
  backend/           # Axum (Rust) backend
    src/
      handlers/      # Request handlers
      middleware/     # Auth middleware
      models/        # SeaORM entity models
      routes/        # Route definitions
      migrations/    # Database migrations
      tasks/         # Background tasks (email)
    static/          # Static files (uploaded images)
  frontend/          # Vue 3 frontend
  docker-compose.yml
  Makefile
```

## License

MIT
