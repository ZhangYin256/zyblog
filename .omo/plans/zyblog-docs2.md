# ZYBlog Fixes & Documentation

## TL;DR

> **Quick Summary**: 修复 GitHub OAuth 错误、添加默认管理员密码、编写 TODO 使用说明、编写详尽的工程文档
>
> **Deliverables**:
> - GitHub OAuth 按钮在未配置时隐藏或显示友好提示
> - 默认管理员账户带密码
> - TODO 功能使用说明
> - 详尽的工程文档（架构、设计、模块、函数级别）
>
> **Estimated Effort**: Large
> **Parallel Execution**: YES — 2 waves
> **Critical Path**: T1 → T4

---

## TODOs

### Wave 1: Bug Fixes (parallel)

- [x] 1. Fix GitHub OAuth Error

  **What to do**:
  - 当 `GITHUB_CLIENT_ID` 未配置时，隐藏 GitHub 登录按钮或显示友好提示
  - 后端 `github_login` handler 返回更好的错误信息
  - 前端 Login.vue 检查 GitHub OAuth 是否可用

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `frontend/src/views/Login.vue` — 登录页面
  - `backend/src/handlers/auth.rs:645-668` — github_login handler

  **Acceptance Criteria**:
  - [ ] 未配置 GitHub OAuth 时，按钮隐藏或显示提示
  - [ ] 不再返回 "Internal server error"

- [x] 2. Add Default Admin Password

  **What to do**:
  - 修改 `main.rs` 中的 admin 用户创建逻辑
  - 使用 `ADMIN_KEY` 环境变量作为默认密码（bcrypt 哈希）
  - 如果 `ADMIN_KEY` 未设置，使用默认密码 `admin123`
  - 在启动日志中显示默认密码

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `backend/src/main.rs:198-222` — admin 用户创建逻辑
  - `backend/src/handlers/auth.rs` — 密码哈希逻辑

  **Acceptance Criteria**:
  - [ ] 管理员账户有默认密码
  - [ ] 可以用默认密码登录

- [x] 3. Write TODO Usage Documentation

  **What to do**:
  - 在 `docs/user-guide.md` 中添加 TODO 功能详细说明
  - 说明 #todo 标记的使用方法
  - 说明订阅和通知机制
  - 添加使用示例

  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `docs/user-guide.md` — 现有用户手册
  - `frontend/src/views/Todos.vue` — TODO 管理页面

  **Acceptance Criteria**:
  - [ ] TODO 功能有详细说明
  - [ ] 包含使用示例

### Wave 2: Comprehensive Documentation

- [x] 4. Write Comprehensive Engineering Documentation

  **What to do**:
  - 创建 `docs/architecture.md` — 详尽的工程文档
  - 覆盖：
    - 系统架构总览
    - 技术栈详解
    - 目录结构说明
    - 数据库设计（ER 图、表结构）
    - API 设计原则
    - 认证流程
    - 前端架构（组件、路由、状态管理）
    - 后端架构（Handler、Model、Middleware）
    - 关键模块和函数说明
    - 设计决策和权衡
    - 扩展指南

  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `AGENTS.md` — 项目知识库
  - `backend/AGENTS.md` — 后端知识库
  - `frontend/AGENTS.md` — 前端知识库
  - `docs/api.md` — API 文档
  - `docs/development.md` — 开发指南
  - 所有源代码文件

  **Acceptance Criteria**:
  - [ ] 文档覆盖所有关键模块
  - [ ] 包含架构图（文字描述）
  - [ ] 包含函数级别说明
  - [ ] 阅读完即可完全掌握工程

---

## Success Criteria

- [ ] GitHub OAuth 未配置时不报错
- [ ] 管理员有默认密码
- [ ] TODO 功能有文档
- [ ] 有详尽的工程文档
