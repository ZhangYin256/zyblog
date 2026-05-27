## docs/api.md 中文化完成

- 保留了所有 API 端点路径和技术细节
- 翻译了所有描述性文本和注释
- 数据类型名称保持英文，添加了中文注释
- 格式和代码块完整保留
## docs/development.md 中文化

- 文件包含完整的开发指南和架构说明
- 翻译时保留了所有技术术语（如 Axum、SeaORM、PostgreSQL）
- 保留了所有代码块和链接格式
- 中文翻译准确，技术细节完整

## 代码注释中文化完成

- 后端 Rust 代码注释翻译完成（main.rs, handlers/posts.rs, handlers/subscribers.rs, handlers/images.rs, middleware/auth.rs, tasks/email.rs）
- 前端 Vue/TS 代码注释翻译完成（api.ts, app.ts, usePosts.ts, Layout.vue, PostDetail.vue, Home.vue, Import.vue, Publish.vue, TodoSubscribe.vue）
- 保留了所有技术细节和代码逻辑
- 仅翻译注释，未修改任何函数名、变量名或代码逻辑
- 通过 git diff 验证：所有变更均为注释翻译
- CSS 注释也已翻译（如 /* --- Header --- */ → /* --- 头部 --- */）
- 测试环境不可用（无 cargo/npx），但代码逻辑未变更
