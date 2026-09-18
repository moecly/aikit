# aikit CLI 工具箱 v1 计划

## 目标
- 单一 `aikit` 二进制，多级子命令：`tr`（翻译）、`commit`（未提交内容生成 commit 建议）、
  `doctor`（显示当前配置并检查 API 连通性）、`models`（列出可用模型）。
- OpenAI 兼容 Chat Completions 后端，环境变量配置；`.envrc` 提交仓库并引入被屏蔽的 `.env.secrets`。

## 步骤
1. [x] 规划（本文件）
2. [x] 从 `main` 建分支 `feat/cli-toolbox`
3. [x] 实现 CLI：`tr`（参数为主+管道，默认中英互译）、`commit`（默认全部未提交，只建议+`--apply` 可选提交）
4. [x] 接 OpenAI 兼容 API（`AIKIT_BASE_URL` / `AIKIT_API_KEY` / `AIKIT_MODEL`）
5. [x] `doctor` / `models`：配置展示、连通性检查、模型列表
6. [x] `.envrc` + `.env.secrets` + `.gitignore`
7. [x] `cargo test` 相关单测、`cargo clippy` 通过（8 单测，0 警告）
8. [ ] 收尾检查 `AGENTS.md`（涉及架构/命令/结构则更新）

## 接口
- `aikit tr [TEXT...] [--to LANG] [--from LANG] [--model M]`
- `aikit commit [--apply] [--all|--staged|--unstaged] [--model M]`（打印建议后 TTY 下交互式确认，`--apply` 免确认）
- `aikit doctor` / `aikit models`
- commit 输出遵循 Conventional Commits（subject 祈使句小写 ≤50 字符，body 写 why）。消息语言由 `AIKIT_COMMIT_LANG` 控制。
- 环境变量：`AIKIT_BASE_URL` / `AIKIT_API_KEY` / `AIKIT_MODEL` / `AIKIT_TR_TO` / `AIKIT_TR_FROM` / `AIKIT_COMMIT_LANG`（flag > 环境变量 > 默认值）；代码内文案统一英文。

## 验收
- 中英互译默认方向正确；管道输入可用。
- 空仓库提示无可提交内容；混合变更输出合法消息；`--apply` 真实提交。
- 无 key 时报错清晰；`doctor` 能判断连通性，`models` 能列出模型。
- 特殊情况：无提交的新仓库 `git diff HEAD` 不可用，已降级为合并暂存区+工作区差异。
