# ZYBlog Phase 2 - 功能完善与全栈跑通

## TL;DR

> **快速摘要**: 将所有文档翻译为中文，实现中优先级功能（PR式互动、自动备份、多媒体支持），并确保数据库、后端、前端全部跑通。
> 
> **交付物**:
> - 所有文档中文版本
> - PR式互动功能
> - 自动备份功能
> - 多媒体支持（视频）
> - 全栈功能验证报告
> 
> **预估工作量**: Large
> **并行执行**: YES - 4 waves
> **关键路径**: Task 1 → Task 5 → Task 9 → Task 13 → F1-F4

---

## Context

### 原始需求
基于 `需求说明.md` 文档，构建个人博客系统。Phase 1 已完成 MVP，Phase 2 将实现更完备的功能。

### Phase 1 完成情况
- ✅ 后端：Axum + SeaORM + PostgreSQL（文章CRUD、图片上传、认证、邮件、导出）
- ✅ 前端：Vue 3 + Naive UI（响应式布局、发布、列表、导入、订阅）
- ✅ 基础设施：Docker Compose、Git、测试（79个通过）

### Phase 2 目标
1. 文档全部中文化
2. 实现中优先级功能
3. 确保全栈功能正常运行

---

## Work Objectives

### 核心目标
完善 ZYBlog 功能，实现需求文档中的中优先级功能，并确保系统稳定运行。

### 具体交付物
- 所有文档中文版本（README、API、开发指南）
- PR式互动功能（读者可对文章提交建议）
- 自动备份功能（数据库定时备份）
- 多媒体支持（视频上传和播放）
- 全栈功能验证报告

### 完成定义
- [ ] 所有文档为中文
- [ ] PR式互动功能可用
- [ ] 自动备份功能可用
- [ ] 视频上传和播放功能可用
- [ ] 数据库、后端、前端全部跑通
- [ ] 所有测试通过

### Must Have
- 所有文档中文版本
- PR式互动功能
- 自动备份功能
- 视频上传和播放
- 全栈功能正常运行

### Must NOT Have (Guardrails)
- ❌ 不实现树状结构（低优先级）
- ❌ 不实现省流模式（低优先级）
- ❌ 不实现痕迹记录（低优先级）
- ❌ 不实现知识试验田（低优先级）
- ❌ 不修改现有 MVP 功能

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** - ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES (Phase 1 已建立)
- **Automated tests**: Tests-after
- **Framework**: Vitest (前端), Rust standard tests (后端)
- **Process**: 先实现功能，再补测试

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.omo/evidence/phase2-task-{N}-{scenario-slug}.{ext}`.

- **Frontend/UI**: Use Playwright - Navigate, interact, assert DOM, screenshot
- **API/Backend**: Use Bash (curl) - Send requests, assert status + response fields
- **Integration**: Use Docker Compose - Full stack testing

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately - 文档中文化):
├── Task 1: README.md 中文化 [quick]
├── Task 2: docs/api.md 中文化 [quick]
├── Task 3: docs/development.md 中文化 [quick]
└── Task 4: 代码注释中文化 [quick]

Wave 2 (After Wave 1 - 后端功能):
├── Task 5: PR式互动 - 数据库设计 (depends: 1) [unspecified-high]
├── Task 6: PR式互动 - API实现 (depends: 5) [unspecified-high]
├── Task 7: 自动备份 - 数据库备份功能 (depends: 1) [unspecified-high]
└── Task 8: 多媒体支持 - 视频上传API (depends: 1) [unspecified-high]

Wave 3 (After Wave 2 - 前端功能):
├── Task 9: PR式互动 - 前端界面 (depends: 6) [visual-engineering]
├── Task 10: 自动备份 - 前端管理界面 (depends: 7) [visual-engineering]
├── Task 11: 多媒体支持 - 视频播放器 (depends: 8) [visual-engineering]
└── Task 12: 全栈联调验证 (depends: 9,10,11) [unspecified-high]

Wave 4 (After Wave 3 - 测试与文档):
├── Task 13: 测试补充 (depends: 12) [unspecified-high]
└── Task 14: 文档更新 (depends: 12) [writing]

Wave FINAL (After ALL tasks — 4 parallel reviews):
├── Task F1: 计划合规审计 (oracle)
├── Task F2: 代码质量审查 (unspecified-high)
├── Task F3: 全栈功能验证 (unspecified-high)
└── Task F4: 范围保真检查 (deep)
-> Present results -> Get explicit user okay

Critical Path: Task 1 → Task 5 → Task 6 → Task 9 → Task 12 → Task 13 → F1-F4
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 4 (Waves 1 & 2)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|------------|--------|
| 1 | - | 2,3,4,5,7,8 |
| 2 | 1 | - |
| 3 | 1 | - |
| 4 | 1 | - |
| 5 | 1 | 6 |
| 6 | 5 | 9 |
| 7 | 1 | 10 |
| 8 | 1 | 11 |
| 9 | 6 | 12 |
| 10 | 7 | 12 |
| 11 | 8 | 12 |
| 12 | 9,10,11 | 13,14 |
| 13 | 12 | F1-F4 |
| 14 | 12 | F1-F4 |
| F1-F4 | 13,14 | user okay |

### Agent Dispatch Summary

- **Wave 1**: 4 tasks - T1-T4 → `quick`
- **Wave 2**: 4 tasks - T5-T8 → `unspecified-high`
- **Wave 3**: 4 tasks - T9-T12 → `visual-engineering` / `unspecified-high`
- **Wave 4**: 2 tasks - T13-T14 → `unspecified-high` / `writing`
- **FINAL**: 4 tasks - F1 → `oracle`, F2-F4 → `unspecified-high` / `deep`

---

## TODOs

- [x] 1. README.md 中文化

- [x] 2. docs/api.md 中文化

- [x] 3. docs/development.md 中文化

- [x] 4. 代码注释中文化

- [x] 5. 本地环境一键部署配置

  **What to do**:
  - 配置本机环境使应用可以一键部署
  - 创建 .env 文件（从 .env.example 复制）
  - 配置 PostgreSQL 数据库连接
  - 配置管理员密钥
  - 配置 SMTP 邮件服务（可选）
  - 创建一键启动脚本
  - 验证 Docker Compose 可以正常启动

  **Must NOT do**:
  - 不要修改现有代码
  - 不要添加云服务配置

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: 环境配置任务，简单直接
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 6, 7, 8, 9)
  - **Blocks**: Task 13
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - 现有 .env.example 文件
  - 现有 docker-compose.yml 文件

  **External References**:
  - Docker Compose 文档

  **Acceptance Criteria**:

  ```
  Scenario: .env 文件配置成功
    Tool: Bash
    Preconditions: .env.example 文件存在
    Steps:
      1. cp .env.example .env
      2. 检查 .env 文件内容
    Expected Result: .env 文件存在且包含必要配置
    Evidence: .omo/evidence/phase2-task-5-env-config.txt

  Scenario: Docker Compose 启动成功
    Tool: Bash
    Preconditions: Docker 已安装
    Steps:
      1. docker-compose up -d
      2. docker-compose ps
    Expected Result: 所有服务运行正常
    Evidence: .omo/evidence/phase2-task-5-docker-start.txt

  Scenario: 数据库连接正常
    Tool: Bash
    Preconditions: PostgreSQL 服务运行
    Steps:
      1. docker-compose exec postgres psql -U zyblog -d zyblog -c "SELECT 1"
    Expected Result: 返回 1
    Evidence: .omo/evidence/phase2-task-5-db-connection.txt

  Scenario: 后端 API 正常
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. curl http://localhost:8080/api/health
    Expected Result: 返回 200，body 包含 "status": "ok"
    Evidence: .omo/evidence/phase2-task-5-backend-api.txt

  Scenario: 前端页面正常
    Tool: Bash (curl)
    Preconditions: 前端服务运行
    Steps:
      1. curl http://localhost:5173
    Expected Result: 返回 HTML 页面
    Evidence: .omo/evidence/phase2-task-5-frontend-page.txt
  ```

  **Commit**: YES
  - Message: `feat: 配置本地一键部署环境`
  - Files: `.env`, `Makefile`, `scripts/`
  - Pre-commit: N/A

- [x] 6. PR式互动 - 数据库设计

  **What to do**:
  - 设计 PR式互动 数据库 schema
  - 创建 pull_requests 表（id, post_id, user_email, content, status, created_at）
  - 创建 pull_request_comments 表（id, pull_request_id, content, created_at）
  - 创建 SeaORM 迁移文件
  - 更新 Model 结构体

  **Must NOT do**:
  - 不要修改现有表结构
  - 不要添加种子数据

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 数据库设计，需要仔细设计
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 7, 8, 9)
  - **Blocks**: Task 7
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - 现有数据库迁移文件：`backend/src/migrations/`

  **External References**:
  - SeaORM 迁移文档

  **Acceptance Criteria**:

  ```
  Scenario: PR式互动数据库设计成功
    Tool: Bash
    Preconditions: PostgreSQL 已启动
    Steps:
      1. cd backend && cargo run -- migrate
      2. psql -d zyblog -c "\dt"
    Expected Result: 迁移成功，新表已创建
    Evidence: .omo/evidence/phase2-task-6-pr-db.txt

  Scenario: Model 结构体可编译
    Tool: Bash
    Preconditions: SeaORM model 已生成
    Steps:
      1. cargo check
    Expected Result: 编译成功，无错误
    Evidence: .omo/evidence/phase2-task-6-pr-models.txt
  ```

  **Commit**: YES
  - Message: `feat(db): 设计 PR式互动 数据库`
  - Files: `backend/src/migrations/`, `backend/src/models/`
  - Pre-commit: `cargo check`

- [x] 7. PR式互动 - API实现

  **What to do**:
  - 实现 PR式互动 API
  - `POST /api/v1/posts/:id/pulls` - 创建 PR
  - `GET /api/v1/posts/:id/pulls` - 获取 PR 列表
  - `PUT /api/v1/pulls/:id` - 更新 PR 状态
  - `POST /api/v1/pulls/:id/comments` - 添加评论
  - 实现 PR 状态管理（open, closed, merged）

  **Must NOT do**:
  - 不要修改现有 API
  - 不要添加复杂权限控制

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 核心业务逻辑，需要仔细设计
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 8, 9)
  - **Blocks**: Task 10
  - **Blocked By**: Task 6

  **References**:

  **Pattern References**:
  - 现有文章 CRUD API：`backend/src/handlers/posts.rs`

  **External References**:
  - Axum 路由文档

  **Acceptance Criteria**:

  ```
  Scenario: 创建 PR 成功
    Tool: Bash (curl)
    Preconditions: 后端服务运行，文章已创建
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/posts/1/pulls \
           -H "Content-Type: application/json" \
           -d '{"user_email":"test@example.com","content":"建议修改标题"}'
    Expected Result: 返回 201，body 包含 PR 信息
    Evidence: .omo/evidence/phase2-task-7-create-pr.txt

  Scenario: 获取 PR 列表
    Tool: Bash (curl)
    Preconditions: PR 已创建
    Steps:
      1. curl http://localhost:8080/api/v1/posts/1/pulls
    Expected Result: 返回 200，body 包含 PR 列表
    Evidence: .omo/evidence/phase2-task-7-list-prs.txt

  Scenario: 添加评论成功
    Tool: Bash (curl)
    Preconditions: PR 已创建
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/pulls/1/comments \
           -H "Content-Type: application/json" \
           -d '{"content":"同意这个建议"}'
    Expected Result: 返回 201，body 包含评论信息
    Evidence: .omo/evidence/phase2-task-7-add-comment.txt
  ```

  **Commit**: YES
  - Message: `feat(api): 实现 PR式互动 API`
  - Files: `backend/src/handlers/pulls.rs`, `backend/src/routes/pulls.rs`
  - Pre-commit: `cargo test`

- [x] 8. 自动备份 - 数据库备份功能

  **What to do**:
  - 实现数据库自动备份功能
  - 创建备份任务（定时执行 pg_dump）
  - 实现备份文件管理（保留最近 N 个备份）
  - 实现备份恢复功能
  - 创建备份配置（备份间隔、保留数量）

  **Must NOT do**:
  - 不要修改现有数据库结构
  - 不要添加云存储支持

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 系统功能，需要仔细设计
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 7, 9)
  - **Blocks**: Task 11
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - 现有后台任务：`backend/src/tasks/email.rs`

  **External References**:
  - PostgreSQL pg_dump 文档

  **Acceptance Criteria**:

  ```
  Scenario: 备份功能可用
    Tool: Bash
    Preconditions: 后端服务运行
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/backup \
           -H "Authorization: Bearer $ADMIN_KEY"
    Expected Result: 返回 200，备份文件已创建
    Evidence: .omo/evidence/phase2-task-8-backup.txt

  Scenario: 备份列表可用
    Tool: Bash (curl)
    Preconditions: 备份已创建
    Steps:
      1. curl http://localhost:8080/api/v1/backup/list \
           -H "Authorization: Bearer $ADMIN_KEY"
    Expected Result: 返回 200，body 包含备份列表
    Evidence: .omo/evidence/phase2-task-8-backup-list.txt
  ```

  **Commit**: YES
  - Message: `feat: 实现自动备份功能`
  - Files: `backend/src/handlers/backup.rs`, `backend/src/tasks/backup.rs`
  - Pre-commit: `cargo test`

- [x] 9. 多媒体支持 - 视频上传API

  **What to do**:
  - 实现视频上传 API
  - `POST /api/v1/videos` - 上传视频
  - 支持格式：mp4, webm, ogg
  - 文件大小限制：100MB
  - 存储路径：`static/videos/`
  - 返回视频 URL

  **Must NOT do**:
  - 不要实现视频转码
  - 不要实现云存储

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 文件上传处理，需要安全考虑
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 5, 6, 7, 8)
  - **Blocks**: Task 12
  - **Blocked By**: Task 1

  **References**:

  **Pattern References**:
  - 现有图片上传 API：`backend/src/handlers/images.rs`

  **External References**:
  - Axum Multipart 文档

  **Acceptance Criteria**:

  ```
  Scenario: 视频上传成功
    Tool: Bash (curl)
    Preconditions: 后端服务运行，static/videos/ 目录存在
    Steps:
      1. curl -X POST http://localhost:8080/api/v1/videos \
           -H "Authorization: Bearer $ADMIN_KEY" \
           -F "file=@test-video.mp4"
    Expected Result: 返回 201，body 包含 url 字段
    Evidence: .omo/evidence/phase2-task-9-upload-success.txt

  Scenario: 视频上传失败（文件过大）
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. dd if=/dev/zero of=large-video.mp4 bs=1M count=200
      2. curl -X POST http://localhost:8080/api/v1/videos \
           -H "Authorization: Bearer $ADMIN_KEY" \
           -F "file=@large-video.mp4"
    Expected Result: 返回 413，body 包含 "file too large" 错误
    Evidence: .omo/evidence/phase2-task-9-upload-too-large.txt
  ```

  **Commit**: YES
  - Message: `feat(api): 实现视频上传 API`
  - Files: `backend/src/handlers/videos.rs`, `backend/src/routes/videos.rs`
  - Pre-commit: `cargo test`

- [x] 10. PR式互动 - 前端界面

  **What to do**:
  - 创建 PR式互动 前端界面
  - 创建 PullRequest.vue 组件
  - 实现 PR 列表显示
  - 实现创建 PR 表单
  - 实现评论功能
  - 集成到文章详情页

  **Must NOT do**:
  - 不要修改现有页面布局
  - 不要添加复杂权限控制

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: 前端界面设计
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: 前端界面设计需要专业指导

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 11, 12, 13)
  - **Blocks**: Task 13
  - **Blocked By**: Task 7

  **References**:

  **Pattern References**:
  - 现有文章详情页：`frontend/src/views/PostDetail.vue`

  **External References**:
  - Naive UI 组件文档

  **Acceptance Criteria**:

  ```
  Scenario: PR 列表显示正确
    Tool: Playwright
    Preconditions: 文章已创建，PR 已创建
    Steps:
      1. 访问文章详情页
      2. 检查 PR 列表
    Expected Result: 显示 PR 列表，包含用户邮箱和内容
    Evidence: .omo/evidence/phase2-task-10-pr-list.png

  Scenario: 创建 PR 成功
    Tool: Playwright
    Preconditions: 文章详情页已打开
    Steps:
      1. 点击创建 PR 按钮
      2. 输入用户邮箱和内容
      3. 点击提交
    Expected Result: PR 创建成功，列表中显示新 PR
    Evidence: .omo/evidence/phase2-task-10-create-pr.png

  Scenario: 添加评论成功
    Tool: Playwright
    Preconditions: PR 已创建
    Steps:
      1. 点击 PR 查看详情
      2. 输入评论内容
      3. 点击提交
    Expected Result: 评论添加成功，显示在评论列表中
    Evidence: .omo/evidence/phase2-task-10-add-comment.png
  ```

  **Commit**: YES
  - Message: `feat(ui): 实现 PR式互动界面`
  - Files: `frontend/src/components/PullRequest.vue`, `frontend/src/views/PostDetail.vue`
  - Pre-commit: `npm run build`

- [x] 11. 自动备份 - 前端管理界面

  **What to do**:
  - 创建备份管理界面
  - 创建 Backup.vue 组件
  - 实现备份列表显示
  - 实现手动备份触发
  - 实现备份恢复功能
  - 添加到导航菜单

  **Must NOT do**:
  - 不要修改现有页面布局
  - 不要添加复杂权限控制

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: 前端界面设计
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: 前端界面设计需要专业指导

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 12, 13)
  - **Blocks**: Task 13
  - **Blocked By**: Task 8

  **References**:

  **Pattern References**:
  - 现有管理界面：`frontend/src/views/Publish.vue`

  **External References**:
  - Naive UI 组件文档

  **Acceptance Criteria**:

  ```
  Scenario: 备份列表显示正确
    Tool: Playwright
    Preconditions: 备份已创建
    Steps:
      1. 访问备份管理页面
      2. 检查备份列表
    Expected Result: 显示备份列表，包含备份时间和大小
    Evidence: .omo/evidence/phase2-task-11-backup-list.png

  Scenario: 手动备份成功
    Tool: Playwright
    Preconditions: 备份管理页面已打开
    Steps:
      1. 点击手动备份按钮
      2. 等待备份完成
    Expected Result: 备份成功，列表中显示新备份
    Evidence: .omo/evidence/phase2-task-11-manual-backup.png

  Scenario: 备份恢复成功
    Tool: Playwright
    Preconditions: 备份已创建
    Steps:
      1. 点击恢复按钮
      2. 确认恢复
    Expected Result: 恢复成功，数据库已恢复
    Evidence: .omo/evidence/phase2-task-11-restore-backup.png
  ```

  **Commit**: YES
  - Message: `feat(ui): 实现备份管理界面`
  - Files: `frontend/src/views/Backup.vue`, `frontend/src/components/Layout.vue`
  - Pre-commit: `npm run build`

- [x] 12. 多媒体支持 - 视频播放器

  **What to do**:
  - 创建视频播放器组件
  - 创建 VideoPlayer.vue 组件
  - 实现视频上传界面
  - 实现视频播放功能
  - 集成到文章编辑器

  **Must NOT do**:
  - 不要修改现有页面布局
  - 不要实现视频转码

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: 前端界面设计
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: 前端界面设计需要专业指导

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 10, 11, 13)
  - **Blocks**: Task 13
  - **Blocked By**: Task 9

  **References**:

  **Pattern References**:
  - 现有图片上传：`frontend/src/views/Publish.vue`

  **External References**:
  - HTML5 video 标签文档

  **Acceptance Criteria**:

  ```
  Scenario: 视频上传成功
    Tool: Playwright
    Preconditions: 发布页面已打开
    Steps:
      1. 点击视频上传按钮
      2. 选择测试视频
      3. 等待上传完成
    Expected Result: 视频上传成功，显示预览
    Evidence: .omo/evidence/phase2-task-12-upload-video.png

  Scenario: 视频播放成功
    Tool: Playwright
    Preconditions: 文章包含视频
    Steps:
      1. 访问文章详情页
      2. 点击播放按钮
    Expected Result: 视频播放成功，控件可用
    Evidence: .omo/evidence/phase2-task-12-play-video.png
  ```

  **Commit**: YES
  - Message: `feat(ui): 实现视频播放器`
  - Files: `frontend/src/components/VideoPlayer.vue`, `frontend/src/views/Publish.vue`
  - Pre-commit: `npm run build`

- [x] 13. 全栈联调验证

  **What to do**:
  - 验证数据库、后端、前端全部跑通
  - 测试所有新功能
  - 修复集成问题
  - 确保系统稳定运行

  **Must NOT do**:
  - 不要修改现有功能
  - 不要添加新功能

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 全栈集成测试
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential
  - **Blocks**: Tasks 14, 15
  - **Blocked By**: Tasks 5, 10, 11, 12

  **References**:

  **Pattern References**:
  - 现有全栈测试：`.omo/evidence/final-qa/`

  **External References**:
  - 无

  **Acceptance Criteria**:

  ```
  Scenario: 数据库连接正常
    Tool: Bash
    Preconditions: PostgreSQL 已启动
    Steps:
      1. psql -d zyblog -c "SELECT 1"
    Expected Result: 返回 1
    Evidence: .omo/evidence/phase2-task-13-db-connection.txt

  Scenario: 后端 API 正常
    Tool: Bash (curl)
    Preconditions: 后端服务运行
    Steps:
      1. curl http://localhost:8080/api/health
    Expected Result: 返回 200，body 包含 "status": "ok"
    Evidence: .omo/evidence/phase2-task-13-backend-api.txt

  Scenario: 前端页面正常
    Tool: Playwright
    Preconditions: 前端服务运行
    Steps:
      1. 访问 http://localhost:5173
      2. 检查页面加载
    Expected Result: 页面加载正常，无错误
    Evidence: .omo/evidence/phase2-task-13-frontend-page.png

  Scenario: 新功能可用
    Tool: Playwright
    Preconditions: 前后端服务运行
    Steps:
      1. 测试 PR式互动功能
      2. 测试自动备份功能
      3. 测试视频上传功能
    Expected Result: 所有新功能可用
    Evidence: .omo/evidence/phase2-task-13-new-features.png
  ```

  **Commit**: YES
  - Message: `fix: 全栈联调验证`
  - Files: 根据需要修改
  - Pre-commit: `cargo test && vitest run`

- [x] 14. 测试补充

  **What to do**:
  - 补充新功能的单元测试
  - 补充集成测试
  - 配置测试覆盖率报告
  - 确保所有测试通过

  **Must NOT do**:
  - 不要修改现有测试
  - 不要追求 100% 覆盖率

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: 测试编写，需要理解业务逻辑
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Task 15)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 13

  **References**:

  **Pattern References**:
  - 现有测试文件：`backend/tests/`, `frontend/src/__tests__/`

  **External References**:
  - Vitest 官方文档
  - Rust 测试文档

  **Acceptance Criteria**:

  ```
  Scenario: 后端测试通过
    Tool: Bash
    Preconditions: 测试已编写
    Steps:
      1. cd backend && cargo test
    Expected Result: 所有测试通过
    Evidence: .omo/evidence/phase2-task-14-backend-tests.txt

  Scenario: 前端测试通过
    Tool: Bash
    Preconditions: 测试已编写
    Steps:
      1. cd frontend && vitest run
    Expected Result: 所有测试通过
    Evidence: .omo/evidence/phase2-task-14-frontend-tests.txt
  ```

  **Commit**: YES
  - Message: `test: 补充新功能测试`
  - Files: `backend/tests/`, `frontend/src/__tests__/`
  - Pre-commit: `cargo test && vitest run`

- [x] 15. 文档更新

  **What to do**:
  - 更新 README.md 添加新功能说明
  - 更新 docs/api.md 添加新 API 端点
  - 更新 docs/development.md 添加新功能开发指南
  - 确保所有文档为中文

  **Must NOT do**:
  - 不要删除现有文档
  - 不要修改代码

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: 文档编写
  - **Skills**: []
    - 无需特殊技能

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Task 14)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 13

  **References**:

  **Pattern References**:
  - 现有文档：`README.md`, `docs/api.md`, `docs/development.md`

  **External References**:
  - 无

  **Acceptance Criteria**:

  ```
  Scenario: README.md 更新成功
    Tool: Read
    Preconditions: README.md 文件存在
    Steps:
      1. 读取 README.md 文件
      2. 检查是否包含新功能说明
    Expected Result: README.md 包含新功能说明
    Evidence: .omo/evidence/phase2-task-15-readme-update.txt

  Scenario: docs/api.md 更新成功
    Tool: Read
    Preconditions: docs/api.md 文件存在
    Steps:
      1. 读取 docs/api.md 文件
      2. 检查是否包含新 API 端点
    Expected Result: docs/api.md 包含新 API 端点
    Evidence: .omo/evidence/phase2-task-15-api-update.txt
  ```

  **Commit**: YES
  - Message: `docs: 更新文档添加新功能说明`
  - Files: `README.md`, `docs/api.md`, `docs/development.md`
  - Pre-commit: N/A

---

## Final Verification Wave

- [x] F1. **计划合规审计** — `oracle`
  读取计划文件，验证所有 Must Have 项是否实现，所有 Must NOT Have 项是否存在。检查证据文件。比较交付物与计划。
  输出: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **代码质量审查** — `unspecified-high`
  运行 `cargo test` + `vitest run` + 代码检查。检查所有更改的文件：`unwrap()` 生产代码、空 catch、console.log、注释代码、未使用导入。检查 AI 代码味道。
  输出: `Build [PASS/FAIL] | Tests [N pass/N fail] | Files [N clean/N issues] | VERDICT`

- [x] F3. **全栈功能验证** — `unspecified-high`
  从干净状态开始。执行每个任务的 QA 场景。测试跨任务集成。测试边界情况。保存到 `.omo/evidence/phase2-final-qa/`。
  输出: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [x] F4. **范围保真检查** — `deep`
  对于每个任务：读取"做什么"，读取实际 diff。验证 1:1 匹配。检查 Must NOT do 合规性。检测跨任务污染。标记未 account 的更改。
  输出: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Wave 1**: `docs: 中文化文档` - README, API, 开发指南
- **Wave 2**: `feat: PR式互动、自动备份、视频上传` - 数据库、API、服务
- **Wave 3**: `feat: 前端功能完善` - 互动界面、备份管理、视频播放
- **Wave 4**: `test: 测试补充` - 单元测试、集成测试
- **Final**: `chore: 最终验证和清理`

---

## Success Criteria

### Verification Commands
```bash
# 启动完整环境
docker-compose up -d

# 后端测试
cargo test

# 前端测试
vitest run

# 全栈验证
curl http://localhost:8080/api/health
curl http://localhost:5173
```

### Final Checklist
- [ ] 所有文档为中文
- [ ] PR式互动功能可用
- [ ] 自动备份功能可用
- [ ] 视频上传和播放功能可用
- [ ] 数据库、后端、前端全部跑通
- [ ] 所有测试通过
