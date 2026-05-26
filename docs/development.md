# Development Guide

## Setup

### Prerequisites

- Docker + Docker Compose
- (Optional) Rust 1.75+ with cargo
- (Optional) Node.js 20+ with npm

### Getting Started

```bash
# Start everything
cp .env.example .env
# Edit .env with your settings
docker-compose up -d
make migrate
```

### Running Tests

```bash
make test
```

## Architecture

### Backend (Axum)

The backend is structured as:

```
src/
  main.rs          # Entry point, router setup, OpenAPI config
  config.rs        # Environment variable loading
  state.rs         # Shared application state
  error.rs         # Error types and HTTP responses
  handlers/        # Request handlers (business logic)
    posts.rs       # CRUD for blog posts + #todo parsing
    subscribers.rs # Newsletter subscriber management
    export.rs      # Data export (JSON/CSV)
    images.rs      # Image upload/serving
  routes/          # Axum router definitions
    posts.rs
    subscribers.rs
    export.rs
    images.rs
  middleware/
    auth.rs        # Bearer token authentication
  models/          # SeaORM entity definitions
    post.rs
    subscriber.rs
    todo_item.rs
    image.rs
  migrations/      # Database migrations
  tasks/
    email.rs       # Email notification tasks
```

### Database

PostgreSQL with SeaORM. Migrations run via `cargo run -- migrate`.

Tables:
- `posts` - Blog posts with title, slug, content, published flag
- `subscribers` - Newsletter subscribers
- `todo_items` - Todo items parsed from `#todo` tags in post content
- `images` - Uploaded image metadata

### Authentication

Admin-only write operations use Bearer token auth. The token is compared against the `ADMIN_KEY` environment variable. Read operations (GET) are public.

### #todo System

Posts can contain `#todo` tags in their content. When a post is created or updated, the system:

1. Parses all `#todo Task description` patterns from the content
2. Creates `todo_item` records linked to the post
3. On update, old todos are replaced with newly parsed ones

## Adding a New Endpoint

1. Create handler in `src/handlers/your_module.rs`
2. Add `pub mod your_module;` to `src/handlers/mod.rs`
3. Create route in `src/routes/your_module.rs`
4. Add `pub mod your_module;` to `src/routes/mod.rs`
5. Wire the route in `src/main.rs`
6. Add utoipa annotations for OpenAPI docs
7. Register paths and schemas in the `ApiDoc` struct

## OpenAPI Documentation

API documentation is auto-generated from code using [utoipa](https://docs.rs/utoipa).

- Add `#[derive(utoipa::ToSchema)]` to request/response types
- Add `#[utoipa::path(...)]` to handler functions
- Register paths and components in the `ApiDoc` struct in `main.rs`

The Swagger UI is served at `/swagger-ui/` and the raw OpenAPI JSON at `/api-docs/openapi.json`.

## Environment Variables

See `.env.example` for all available configuration options.
