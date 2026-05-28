# ZYBlog Role-Based UI + Admin Features

## TL;DR

> **Quick Summary**: 基于角色的侧边栏/底边栏重构、管理员删除+回收站、用户信息编辑、路由权限控制
>
> **Deliverables**:
> - 角色化的侧边栏菜单（访客/用户/管理员）
> - 角色化的移动端底边栏
> - 管理员：删除文章、删除备份、回收站
> - 用户信息界面（编辑邮箱、昵称）
> - 路由权限控制（URL 直接访问拦截）
>
> **Estimated Effort**: Large
> **Parallel Execution**: YES — 3 waves
> **Critical Path**: T1 → T2 → T6

---

## Context

### Original Request
用户请求：
1. 管理员删除文章/备份 + 回收站
2. 用户信息界面（编辑邮箱、昵称）
3. 侧边栏角色化（访客/用户/管理员不同菜单）
4. 移动端底边栏优化
5. 路由权限控制

### 角色定义
- **访客（未登录）**: 首页、文章列表、登录
- **普通用户**: 首页、文章列表、我的订阅、我的PR、我的评论、个人中心
- **管理员**: 首页、发布文章、草稿箱、导入、数据备份、评论管理、媒体管理、TODO管理、标签管理、回收站

---

## TODOs

### Wave 1: Backend Features

- [x] 1. Add Soft Delete to Posts

  **What to do**:
  - 添加 migration: `deleted_at TIMESTAMPTZ` 到 posts 表
  - 更新 `handlers/posts.rs`:
    - `DELETE /api/v1/posts/:id` 设置 `deleted_at` 而非物理删除
    - 添加 `GET /api/v1/posts/trash` — 回收站列表（admin only）
    - 添加 `POST /api/v1/posts/:id/restore` — 恢复文章（admin only）
    - 添加 `DELETE /api/v1/posts/:id/permanent` — 永久删除（admin only）
  - 更新 `list_posts` 过滤掉 `deleted_at IS NOT NULL` 的文章

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Task 6 (Recycle bin UI)
  - **Blocked By**: None

  **References**:
  - `backend/src/handlers/posts.rs` — 文章 handler
  - `backend/src/models/post.rs` — 文章模型
  - `backend/src/migrations/` — 迁移模式

  **Acceptance Criteria**:
  - [ ] posts 表有 deleted_at 字段
  - [ ] 删除文章设置 deleted_at
  - [ ] 回收站 API 返回已删除文章
  - [ ] 恢复和永久删除 API 工作

- [x] 2. Add User Profile API

  **What to do**:
  - 添加 `PUT /api/v1/auth/profile` — 更新用户信息（邮箱、昵称）
  - 添加 `PUT /api/v1/auth/password` — 修改密码
  - 请求体: `{ "email": "...", "name": "..." }`

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Task 9 (User profile UI)
  - **Blocked By**: None

  **References**:
  - `backend/src/handlers/auth.rs` — 认证 handler
  - `backend/src/models/user.rs` — 用户模型

  **Acceptance Criteria**:
  - [ ] 更新邮箱和昵称 API 工作
  - [ ] 修改密码 API 工作
  - [ ] utoipa 注解完整

- [x] 3. Add Backup Delete API

  **What to do**:
  - 添加 `DELETE /api/v1/backup/:filename` — 删除备份文件（admin only）
  - 验证文件存在且在备份目录内（防止路径遍历）

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Task 8 (Backup management UI)
  - **Blocked By**: None

  **References**:
  - `backend/src/handlers/backup.rs` — 备份 handler

  **Acceptance Criteria**:
  - [ ] 删除备份文件 API 工作
  - [ ] 路径遍历防护

### Wave 2: Frontend Features

- [x] 4. Redesign Article List Page

  **What to do**:
  - 重构 `views/Home.vue` 或创建新的 `views/ArticleList.vue`
  - 文章列表页面**不显示** Hero 动画区域（与首页区分）
  - 支持按时间排序（最新/最早）
  - 支持按标签筛选（点击标签过滤文章）
  - 标签筛选 + 时间排序组合使用
  - 保持搜索功能

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 7, 8, 9)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `frontend/src/views/Home.vue` — 当前首页（有 Hero）
  - `frontend/src/composables/usePosts.ts` — 文章 composable
  - `frontend/src/composables/useTags.ts` — 标签 composable

  **Acceptance Criteria**:
  - [ ] 文章列表页面无 Hero 动画
  - [ ] 可以按时间排序（最新/最早）
  - [ ] 可以按标签筛选
  - [ ] 标签筛选和排序可以组合使用

- [x] 5. Redesign Sidebar with Role-Based Menus

  **What to do**:
  - 重构 `Layout.vue` 的 `menuOptions`：
    - 访客: 首页、文章列表、登录
    - 普通用户: 首页、文章列表、我的订阅、我的PR、我的评论、个人中心
    - 管理员: 首页、发布文章、草稿箱、导入、数据备份、评论管理、媒体管理、TODO管理、标签管理、回收站
  - 使用 `computed` 根据 `user.role` 动态生成菜单
  - 侧边栏底部显示用户信息和登出按钮

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 7, 8)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `frontend/src/components/Layout.vue` — 布局组件
  - `frontend/src/composables/useAuth.ts` — 认证状态

  **Acceptance Criteria**:
  - [ ] 访客看到：首页、文章列表、登录
  - [ ] 普通用户看到：首页、文章列表、我的订阅、我的PR、我的评论、个人中心
  - [ ] 管理员看到：所有管理功能
  - [ ] 侧边栏底部有登出按钮

- [x] 6. Redesign Mobile Bottom Nav

  **What to do**:
  - 重构移动端底边栏：
    - 访客: 首页、文章列表、登录
    - 普通用户: 首页、文章列表、我的（子页面）
    - 管理员: 首页、发布文章、管理（子页面）
  - "我的"和"管理"点击展开子菜单

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 6, 7, 8)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `frontend/src/components/Layout.vue` — 底边栏

  **Acceptance Criteria**:
  - [ ] 访客底边栏：首页、文章列表、登录
  - [ ] 用户底边栏：首页、文章列表、我的
  - [ ] 管理员底边栏：首页、发布文章、管理
  - [ ] 子菜单展开正常

- [x] 7. Add Recycle Bin Page

  **What to do**:
  - 创建 `views/Trash.vue` — 回收站页面
  - 显示已删除文章列表
  - 恢复按钮
  - 永久删除按钮（带确认）
  - 添加路由 `/trash`

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5, 7, 8)
  - **Blocks**: None
  - **Blocked By**: Task 1

  **References**:
  - `frontend/src/views/Drafts.vue` — 列表页面模式
  - `frontend/src/composables/usePosts.ts` — 文章 composable

  **Acceptance Criteria**:
  - [ ] 回收站页面显示已删除文章
  - [ ] 恢复功能工作
  - [ ] 永久删除功能工作

- [x] 8. Add User Profile Page

  **What to do**:
  - 创建 `views/Profile.vue` — 用户信息页面
  - 编辑邮箱、昵称表单
  - 修改密码表单
  - 添加路由 `/profile`

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5, 6, 8)
  - **Blocks**: None
  - **Blocked By**: Task 2

  **References**:
  - `frontend/src/views/Login.vue` — 表单模式
  - `frontend/src/composables/useAuth.ts` — 认证 composable

  **Acceptance Criteria**:
  - [ ] 用户信息页面显示当前信息
  - [ ] 可以编辑邮箱和昵称
  - [ ] 可以修改密码

- [x] 9. Update Backup Management UI

  **What to do**:
  - 更新 `views/Backup.vue` — 添加删除备份按钮
  - 添加确认对话框

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5, 6, 7)
  - **Blocks**: None
  - **Blocked By**: Task 3

  **References**:
  - `frontend/src/views/Backup.vue` — 备份页面

  **Acceptance Criteria**:
  - [ ] 删除备份按钮可见
  - [ ] 确认对话框工作
  - [ ] 删除后列表更新

### Wave 3: Route Protection + Integration

- [x] 10. Implement Route Permission Guards

  **What to do**:
  - 更新 `router/index.ts` 的路由守卫
  - 定义角色权限映射：
    ```typescript
    const rolePermissions: Record<string, string[]> = {
      visitor: ['/', '/posts', '/posts/:id', '/login'],
      user: ['/', '/posts', '/posts/:id', '/my', '/my/subscriptions', '/my/pulls', '/my/comments', '/profile'],
      admin: ['*'], // 所有路由
    }
    ```
  - 检查用户角色是否有权限访问目标路由
  - 无权限时重定向到首页或登录页

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3
  - **Blocks**: None
  - **Blocked By**: Tasks 4, 5

  **References**:
  - `frontend/src/router/index.ts` — 路由配置
  - `frontend/src/composables/useAuth.ts` — 认证状态

  **Acceptance Criteria**:
  - [ ] 访客无法访问 /publish（重定向到 /login）
  - [ ] 普通用户无法访问 /backup（重定向到首页）
  - [ ] URL 直接访问被正确拦截

- [x] 11. Add "My" Subpages for Regular Users

  **What to do**:
  - 创建 `views/MySubscriptions.vue` — 我的订阅（TODO、标签、新文章通知）
  - 创建 `views/MyPulls.vue` — 我的 PR 列表
  - 创建 `views/MyComments.vue` — 我的评论列表
  - 创建 `views/My.vue` — "我的"父页面（包含子导航）
  - 添加路由

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3
  - **Blocks**: None
  - **Blocked By**: Tasks 4, 5

  **References**:
  - `frontend/src/views/Comments.vue` — 列表页面模式
  - `frontend/src/composables/` — 相关 composable

  **Acceptance Criteria**:
  - [ ] 我的订阅页面显示订阅的 TODO、标签
  - [ ] 我的 PR 页面显示用户提交的 PR
  - [ ] 我的评论页面显示用户的评论

---

## Success Criteria

- [ ] 侧边栏根据角色显示不同菜单
- [ ] 移动端底边栏根据角色显示不同项目
- [ ] 管理员可以删除文章（软删除）和备份
- [ ] 回收站可以恢复和永久删除文章
- [ ] 用户可以编辑个人信息
- [ ] 路由权限控制工作正常
