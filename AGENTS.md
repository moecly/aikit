# 沟通规范

- 始终跟随用户当前使用的语言回复。
- 回复保持简洁，优先提供明确结论和下一步操作。
- 不主动执行高成本操作（如编译、完整测试、构建镜像、大规模扫描等），除非用户明确要求或该操作是完成任务的必要步骤。
- 执行耗时操作前，需要说明原因。

# 工作流程

## 计划管理

- 开发任何功能或修复前，先制定计划。
- 计划必须写入 `plan.md`。
- 按计划执行任务，并在必要时同步更新计划状态。
- 未明确计划前，不主动开始大规模修改。

## 文档维护

- 开发结束后检查并更新项目 `AGENTS.md`。
- 只有涉及以下内容的变更才需要更新文档：
  - 架构
  - 开发约定
  - 命令
  - 文件结构

- 以下情况无需更新：
  - 纯 bug 修复
  - 小范围重构
  - typo 修复

# 代码风格

- 默认不写注释。
- 只有在 WHY 不明显时添加一行注释，用于说明：
  - 隐式约束
  - 特殊设计原因
  - 反直觉行为

- 不写没有必要的抽象。
- 不添加没有实际价值的错误处理。
- 不编写为了不存在场景准备的兼容代码。
- 默认信任内部代码和框架保证。

# Git 规范

## 分支管理

- 开发任何功能或修复前：
  1. 从最新 `main/master` 创建新分支。
  2. 禁止直接在 `main/master` 上提交。

- 分支命名：

  `<type>/<简短描述>`

示例：

  `feat/oauth-login`
  `fix/memory-leak`

规则：

- 全小写。
- 使用 kebab-case。
- 不包含中文。

## 提交格式

使用 Conventional Commits：

    <type>(<scope>): <subject>

    <body>

规则：

- type：
  - `feat`
  - `fix`
  - `refactor`
  - `perf`
  - `chore`
  - `docs`
  - `test`
  - `style`
  - `build`
  - `ci`

- scope：
  - 可选。
  - 标识影响模块。

- subject：
  - 使用祈使句。
  - 小写开头。
  - 不加句号。
  - 不超过 50 个字符。

- body：
  - 说明为什么修改（why）。
  - 不描述简单修改内容。
  - 多条原因使用列表。

示例：

    feat(auth): support OAuth2 login

    - 接入第三方登录需求
    - 复用现有 token 刷新机制，避免重复实现

## 合并要求

- 合并前确保测试/lint 通过。
- 不主动执行无必要的验证流程。

## 提交禁令

禁止添加：

- `Co-Authored-By`
- AI 生成器署名。
- 任何形式的 AI 贡献声明。

# 操作原则

- 优先完成用户目标，避免执行无关操作。
- 不主动运行高成本命令：
  - 编译
  - 全量测试
  - 构建
  - 依赖安装
  - 性能分析
  - 大规模扫描

- 只有以下情况执行：
  - 用户明确要求。
  - 修改必须依赖该结果。
  - 用于定位明确的问题。

- 不确定是否需要执行耗时操作时，先询问用户。

# 安全红线

- 禁止读取 `.env.secrets`（存真实 API key 及其他密钥），也不得以任何形式打印、转述其中的密钥内容。
- 需验证 key 相关链路时，用占位值（如 `AIKIT_API_KEY=dummy`）跑到网络层为止，或请用户自行运行验证。
- 不执行会加载真实密钥的命令（如 `source .env.secrets`、`direnv export`）。

# aikit 项目说明

- Rust 单一二进制，`cargo build` 产物为 `aikit`。
- 命令：`aikit tr`（翻译）、`aikit commit`（未提交内容生成 commit 建议，打印后交互式确认提交，`--apply` 跳过确认直接提交）、
  `aikit doctor`（显示配置并检查连通性）、`aikit models`（列出可用模型）。
- 大模型后端为 OpenAI 兼容 API，配置经环境变量：`AIKIT_BASE_URL` / `AIKIT_API_KEY` / `AIKIT_MODEL`，另有 `AIKIT_TR_TO` / `AIKIT_TR_FROM`（翻译默认目标/源语言，不设则自动中英互译）与 `AIKIT_COMMIT_LANG`（commit 消息语言，默认 English）；命令行 flag 优先于环境变量。
- 代码内用户可见文案统一用英文。
- `.envrc` 可提交，提供默认值并引入 `.env.secrets`；`.env.secrets` 存 key，已屏蔽，不得提交。
- 源码：`src/main.rs` 入口，`cli`/`config`/`llm` 基础模块，`cmd_tr`/`cmd_commit`/`cmd_doctor` 各子命令。
