EVIDENCE: README.md updated to reflect local PostgreSQL development

Date: 2026-05-27
Task: Update README.md to prioritize local development over Docker

Changes made:
1. Added "安装 PostgreSQL" section with Ubuntu/Debian install commands
2. Restructured "本地开发（推荐）" section - removed redundant "启动 PostgreSQL" step (moved to install section)
3. Expanded "一键启动脚本" section with detailed run.txt content explanation
4. Moved Docker section after local dev and run.txt, with note "如果不希望本地安装 PostgreSQL"
5. Fixed DATABASE_URL default value: changed `postgres:5432` to `localhost:5432`

Verification:
- README.md contains "安装 PostgreSQL" section: YES
- README.md contains "本地开发（推荐）" as primary approach: YES
- README.md contains "一键启动脚本" with run.txt details: YES
- README.md contains "使用 Docker（可选）" as optional: YES
- DATABASE_URL default uses localhost: YES
