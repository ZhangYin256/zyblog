# ZYBlog Bug Fixes & UX Improvements

## TL;DR

> **Quick Summary**: 修复 PR 提交无反应、添加文章编辑入口、导入标题可编辑、添加登录入口、添加标签管理、评论引用内容不变性
>
> **Deliverables**:
> - PR 提交功能正常工作
> - 文章详情页有编辑按钮（管理员可见）
> - 导入页面标题可编辑（默认值为自动标题逻辑）
> - 侧边栏/头部有登录入口
> - 标签管理页面
> - 评论引用内容快照（PR 后引用不变）
>
> **Estimated Effort**: Medium
> **Parallel Execution**: YES — 3 waves
> **Critical Path**: T1 → T3 → T5

---

## Context

### Original Request
用户报告了 6 个问题：
1. 提交 PR 没有反应
2. 文章编辑功能没有入口
3. 导入 markdown 时应提供编辑标题功能
4. 没有账号登录入口
5. 没有标签管理功能
6. 评论引用内容需要保证不变性

---

## TODOs

### Wave 1: Quick Fixes

- [x] 1. Fix PR Submission

  **What to do**:
  - 检查 `PullRequest.vue` 中的 `handleCreatePull` 函数
  - 检查 `usePullRequests.ts` 中的 `createPull` 函数
  - 检查后端 `handlers/pulls.rs` 中的 `create_pull` 端点
  - 修复 PR 提交无反应的问题

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: Task 6 (Comment referencing)
  - **Blocked By**: None

  **References**:
  - `frontend/src/components/PullRequest.vue` — PR 组件
  - `frontend/src/composables/usePullRequests.ts` — PR composable
  - `backend/src/handlers/pulls.rs` — PR API handler

  **Acceptance Criteria**:
  - [ ] 选中文字后可以创建 PR
  - [ ] PR 列表正确显示
  - [ ] 管理员可以应用 PR

- [x] 2. Add Edit Button on Post Detail

  **What to do**:
  - 在 `PostDetail.vue` 的文章头部添加"编辑"按钮
  - 仅管理员可见（`isAuthenticated` 检查）
  - 点击跳转到 `/publish/:id`
  - 使用 Naive UI `NButton` 组件

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `frontend/src/views/PostDetail.vue` — 文章详情页
  - `frontend/src/views/Publish.vue` — 发布页（已有编辑模式）

  **Acceptance Criteria**:
  - [ ] 管理员可以看到编辑按钮
  - [ ] 点击编辑按钮跳转到 `/publish/:id`
  - [ ] 非管理员看不到编辑按钮

- [x] 3. Make Import Title Editable

  **What to do**:
  - 在 `Import.vue` 的预览区域，将标题从 `<h3>` 改为可编辑的 `NInput`
  - 默认值为自动标题逻辑（frontmatter > H1 > 文件名 > 'Untitled'）
  - 用户可以修改标题后再导入

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `frontend/src/views/Import.vue` — 导入页面

  **Acceptance Criteria**:
  - [ ] 标题输入框默认值为自动标题
  - [ ] 用户可以修改标题
  - [ ] 导入时使用用户修改后的标题

- [x] 4. Add Login Entry Point

  **What to do**:
  - 在 `Layout.vue` 的侧边栏底部添加"登录"按钮
  - 未登录时显示"登录"，已登录时显示用户名
  - 点击"登录"跳转到 `/login`
  - 添加到移动端头部

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `frontend/src/components/Layout.vue` — 布局组件
  - `frontend/src/composables/useAuth.ts` — 认证 composable

  **Acceptance Criteria**:
  - [ ] 侧边栏有登录按钮
  - [ ] 已登录时显示用户名
  - [ ] 点击登录跳转到 `/login`

### Wave 2: Tag Management

- [x] 5. Add Tag Management Page

  **What to do**:
  - 创建 `frontend/src/views/Tags.vue` — 标签管理页面
  - 显示所有标签列表
  - 创建新标签表单
  - 删除标签功能
  - 添加路由 `/tags` 到 `router/index.ts`
  - 在 `Layout.vue` 侧边栏添加"标签管理"菜单项

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `frontend/src/composables/useTags.ts` — 标签 composable
  - `frontend/src/views/Comments.vue` — 管理页面模式
  - `frontend/src/router/index.ts` — 路由配置
  - `frontend/src/components/Layout.vue` — 侧边栏菜单

  **Acceptance Criteria**:
  - [ ] 标签管理页面显示所有标签
  - [ ] 可以创建新标签
  - [ ] 可以删除标签
  - [ ] 侧边栏有标签管理入口

### Wave 3: Comment Referencing

- [x] 6. Add Comment Content Referencing

  **What to do**:
  - 在 `comments` 表添加 `referenced_content` 字段（TEXT，可空）
  - 在评论提交时，如果用户选中了文章内容，将选中内容存储为快照
  - 在评论显示时，如果有引用内容，显示为引用块样式
  - 使用 migration 添加字段

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3
  - **Blocks**: None
  - **Blocked By**: Task 1 (PR submission fix)

  **References**:
  - `backend/src/models/comment.rs` — 评论模型
  - `backend/src/handlers/comments.rs` — 评论 handler
  - `frontend/src/views/PostDetail.vue` — 评论提交
  - `backend/src/migrations/` — 迁移模式

  **Acceptance Criteria**:
  - [ ] `comments` 表有 `referenced_content` 字段
  - [ ] 评论提交时可以附带引用内容
  - [ ] 评论显示时引用内容以引用块样式展示
  - [ ] PR 应用后引用内容不变

---

## Success Criteria

- [ ] PR 提交功能正常
- [ ] 文章详情页有编辑按钮
- [ ] 导入页面标题可编辑
- [ ] 侧边栏有登录入口
- [ ] 标签管理页面可用
- [ ] 评论引用内容不变性
