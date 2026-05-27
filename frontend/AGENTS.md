# Frontend — Vue 3 Knowledge Base

## OVERVIEW

Vue 3 SPA with Composition API, Vite build, Pinia state, Vue Router, Naive UI components, Vitest tests.

## STRUCTURE

```
src/
├── main.ts           # Bootstrap: createApp + Pinia + Router
├── App.vue           # Root component
├── style.css         # Global styles
├── router/index.ts   # 7 routes, all lazy-loaded
├── stores/app.ts     # Pinia store (single store)
├── lib/api.ts        # Axios instance with auto-auth interceptor
├── composables/      # Reusable composition functions
│   ├── usePosts.ts        # Post CRUD API calls
│   ├── usePullRequests.ts # PR interaction API calls
│   ├── useAuth.ts         # Admin key management
│   └── useResponsive.ts  # Responsive layout helpers
├── components/       # Reusable UI components
│   ├── Layout.vue         # App shell/layout
│   ├── PullRequest.vue    # PR interaction component
│   ├── VideoPlayer.vue    # Video playback
│   ├── TodoSubscribe.vue  # Todo + subscribe widget
│   └── HelloWorld.vue     # Vite scaffold leftover (unused)
├── views/            # Page-level components (route targets)
│   ├── Home.vue           # Post list (/)
│   ├── PostDetail.vue     # Single post (/posts/:id)
│   ├── Publish.vue        # Create/edit post (/publish)
│   ├── Drafts.vue         # Draft posts (/drafts)
│   ├── Import.vue         # Import posts (/import)
│   └── Backup.vue         # Backup management (/backup)
└── __tests__/        # Vitest tests (co-located by type)
    ├── views/
    ├── components/
    └── composables/
```

## WHERE TO LOOK

| Task | File | Notes |
|------|------|-------|
| New page | `views/{Name}.vue` + `router/index.ts` | Add lazy route |
| New API call | `composables/use{Name}.ts` | Import `api` from `lib/api.ts` |
| New component | `components/{Name}.vue` | Naive UI for styling |
| Auth token | `composables/useAuth.ts` | localStorage key: `zyblog_admin_key` |
| Global state | `stores/app.ts` | Pinia store |
| API config | `lib/api.ts` | Axios with auth interceptor |

## CONVENTIONS

- **`<script setup>`** — All components use Composition API with `<script setup lang="ts">`
- **Composables**: API logic lives in `composables/`, not in components
- **Auth**: Token in localStorage, auto-attached to POST/PUT/DELETE by axios interceptor
- **Routes**: All lazy-loaded via `() => import('../views/...')`
- **UI library**: Naive UI — use `n-` prefixed components for consistency
- **Test location**: `__tests__/{views,components,composables}/` (not co-located)
- **Test framework**: Vitest + happy-dom + @vue/test-utils

## ANTI-PATTERNS

- Do NOT put API calls in components — use composables
- Do NOT use `any` type — TypeScript strict
- Do NOT import from `naive-ui` without checking if component exists in their library
- Do NOT add new Pinia stores without checking if `stores/app.ts` can be extended
- Do NOT use Options API — Composition API only

## COMMANDS

```bash
npm run dev          # Vite dev server (port 5173, proxies /api to :8080)
npm run build        # vue-tsc type-check + vite build
npm run test         # Vitest single run
npm run test:watch   # Vitest watch mode
npm run test:coverage # Vitest with coverage
```

## NOTES

- Vite proxies `/api` to `http://localhost:8080` in dev mode
- API base URL is empty string in axios — relies on proxy/same-origin
- `HelloWorld.vue` is unused scaffold leftover
- No e2e tests (no Playwright/Cypress)
- Coverage output in `frontend/coverage/`
