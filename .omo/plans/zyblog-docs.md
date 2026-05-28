# ZYBlog 使用文档

## TL;DR

> **Quick Summary**: 编写完整的中文使用文档，覆盖 API 文档、开发指南、部署指南、用户使用手册四个部分。
>
> **Deliverables**:
> - `docs/api.md` — API 端点文档
> - `docs/development.md` — 开发指南
> - `docs/deployment.md` — 部署指南
> - `docs/user-guide.md` — 用户使用手册
>
> **Estimated Effort**: Medium
> **Parallel Execution**: YES — 4 tasks parallel
> **Critical Path**: All independent

---

## TODOs

- [x] 1. API 文档 (`docs/api.md`)

  **What to do**:
  - 整理所有 API 端点（auth, posts, tags, comments, media, revisions, pulls, todos, backup, export）
  - 每个端点：方法、路径、请求体、响应体、认证要求
  - 认证说明：JWT Bearer token + ADMIN_KEY 双认证
  - 错误码说明
  - 分页参数说明

  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `backend/src/handlers/` — 所有 handler 文件
  - `backend/src/main.rs` — OpenAPI 定义
  - `backend/src/middleware/jwt.rs` — 认证逻辑

  **Acceptance Criteria**:
  - [ ] 所有 API 端点文档化
  - [ ] 包含请求/响应示例
  - [ ] 中文撰写

- [x] 2. 开发指南 (`docs/development.md`)

  **What to do**:
  - 本地开发环境搭建（PostgreSQL, Rust, Node.js）
  - 项目结构说明
  - 启动命令（后端、前端）
  - 数据库迁移命令
  - 测试运行方式
  - 代码规范和约定
  - 新增功能的开发流程

  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `AGENTS.md` — 项目知识库
  - `backend/AGENTS.md` — 后端知识库
  - `frontend/AGENTS.md` — 前端知识库
  - `README.md` — 现有文档
  - `run.txt` — 启动命令

  **Acceptance Criteria**:
  - [ ] 包含环境搭建步骤
  - [ ] 包含启动和测试命令
  - [ ] 中文撰写

- [x] 3. 部署指南 (`docs/deployment.md`)

  **What to do**:
  - Docker Compose 部署方式
  - 环境变量配置说明
  - 数据库迁移步骤
  - 前端 Dockerfile 说明
  - 常见问题排查

  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `docker-compose.yml` — Docker 配置
  - `frontend/Dockerfile` — 前端 Dockerfile
  - `backend/Dockerfile` — 后端 Dockerfile
  - `.env.example` — 环境变量示例
  - `Makefile` — 便捷命令

  **Acceptance Criteria**:
  - [ ] 包含 Docker 部署步骤
  - [ ] 包含环境变量说明
  - [ ] 中文撰写

- [x] 4. 用户使用手册 (`docs/user-guide.md`)

  **What to do**:
  - 注册/登录流程（邮箱密码 + GitHub OAuth）
  - 文章管理（创建、编辑、发布、草稿）
  - 标签管理
  - 评论系统（提交、审核）
  - PR 协作（片段选中、提交 PR、应用 PR）
  - 版本控制（查看历史、回滚、diff）
  - 媒体管理（上传图片/视频）
  - TODO 管理（订阅、完成通知）
  - 搜索功能
  - 主题切换

  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `frontend/src/views/` — 所有页面组件
  - `frontend/src/router/index.ts` — 路由定义
  - `README.md` — 现有功能说明

  **Acceptance Criteria**:
  - [ ] 覆盖所有用户功能
  - [ ] 包含操作步骤说明
  - [ ] 中文撰写

---

## Success Criteria

- [ ] 4 个文档文件创建完成
- [ ] 所有文档使用中文
- [ ] 内容准确反映当前代码库状态
