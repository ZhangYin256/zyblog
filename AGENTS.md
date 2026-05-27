# ZYBlog — Project Knowledge Base

**Generated:** 2026-05-28
**Commit:** b81a0c9
**Branch:** main

## OVERVIEW

Blog platform: Vue 3 + Vite frontend, Axum (Rust) backend, PostgreSQL 15 + SeaORM. REST API at `/api/v1/`. OpenAPI docs via utoipa. Docker Compose for orchestration.

## STRUCTURE

```
zyblog/
├── backend/           # Axum (Rust) — see backend/AGENTS.md
│   ├── src/
│   │   ├── handlers/  # Request handlers (business logic)
│   │   ├── models/    # SeaORM entity definitions
│   │   ├── routes/    # Route group nesting
│   │   ├── middleware/ # Bearer token auth
│   │   ├── migrations/ # SeaORM migrations (7 files)
│   │   └── tasks/     # Background: email, backup
│   ├── tests/         # Integration tests
│   └── static/        # Uploaded images/videos
├── frontend/          # Vue 3 — see frontend/AGENTS.md
│   └── src/
├── docs/              # API + development docs
├── docker-compose.yml # postgres + backend + frontend
├── Makefile           # Docker convenience commands
└── run.txt            # Local dev startup (tmux)
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| New API endpoint | `backend/src/handlers/` + `backend/src/routes/` | Handler + route registration |
| Database schema | `backend/src/models/` + `backend/src/migrations/` | SeaORM entities + migrations |
| New page/route | `frontend/src/views/` + `frontend/src/router/index.ts` | Lazy-loaded Vue components |
| API calls from frontend | `frontend/src/composables/` | `usePosts.ts`, `usePullRequests.ts` |
| Auth logic | `backend/src/middleware/auth.rs` | Bearer token, GET exempt |
| Email notifications | `backend/src/tasks/email.rs` | SMTP via lettre |
| Backup/restore | `backend/src/tasks/backup.rs` | pg_dump shell exec |
| Env config | `backend/src/config.rs` + `.env.example` | dotenvy |

## CONVENTIONS

- **Auth**: Simple Bearer token (`ADMIN_KEY` env var). GET = public, POST/PUT/DELETE = auth required
- **API prefix**: All endpoints under `/api/v1/`
- **Error handling**: `AppError` enum (thiserror) → HTTP status + JSON `{ "error": "..." }`
- **State**: `AppState { db: Option<DatabaseConnection>, config: Config }` wrapped in `Arc`
- **DB optional**: App runs without DB (graceful degradation). `state.db.as_ref().ok_or(...)?`
- **Slugs**: Auto-generated from title via `slug::slugify()`
- **Frontend auth**: Token in `localStorage('zyblog_admin_key')`, auto-attached by axios interceptor
- **Frontend state**: Pinia store in `stores/app.ts`
- **UI library**: Naive UI (not Vuetify/Element)

## ANTI-PATTERNS (THIS PROJECT)

- Do NOT commit `Cargo.lock` (in .gitignore — unusual for binary, but project convention)
- Do NOT use JWT/sessions — auth is single shared `ADMIN_KEY` token
- Do NOT add user accounts — this is a single-admin blog
- Do NOT use `#[tokio::main]` in lib code — only in `main.rs`
- Do NOT hardcode DB connection — always use `state.db.as_ref().ok_or(...)?`

## COMMANDS

```bash
# Local dev
cd backend && DATABASE_URL=... ADMIN_KEY=... cargo run
cd frontend && npm install && npm run dev

# Docker
docker-compose up -d          # Start all
docker-compose down            # Stop all
make migrate                   # Run migrations
make test                      # Backend tests
make db-shell                  # psql access

# Frontend
cd frontend && npm run test    # Vitest
cd frontend && npm run build   # vue-tsc + vite build
```

## NOTES

- No CI/CD pipeline exists — no `.github/workflows/`
- Frontend Dockerfile missing — `docker-compose build` fails for frontend service
- `run.txt` contains hardcoded admin key (dev only)
- Backend tests: `cargo test` (unit in handlers + integration in `tests/`)
- Frontend tests: Vitest + happy-dom + @vue/test-utils
- Video upload limit: 105MB (DefaultBodyLimit in main.rs)
- Swagger UI at `/swagger-ui/` when server running
