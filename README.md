# README

## 文档目的
作为项目入口，说明仓库当前状态与导航路径。

## 当前状态
- 状态：v0.2 实现中（CLI 基线可运行）
- 设计阶段：已冻结
- 实现阶段：进行中（source/update 主链路已打通）

## 上下文输入
- 项目名：Synora
- 类型：AI 驱动的软件操作系统管理器（Draft）
- 当前约束：仅搭建文档与目录，不进入详细设计

## Vision（Draft）
- Synora 终极愿景：`Windows 的 Raycast + Homebrew + AI 安全编排层`。
- 主线 1（入口层）：全局搜索即操作（Raycast 风格）。
- 主线 2（供给层）：公共/个人软件仓库 + `software.yaml`（Homebrew 风格）。
- 主线 3（控制层）：AI 建议 + 安全门禁 + 全链路审计（Synora 差异化）。

## 预期输出
- 读者可快速定位所有核心文档
- 明确当前不包含最终架构/接口结论

## 项目定位决策（Phase 1 Freeze）
- 项目一句话定位：`Synora 是 AI 驱动的软件操作系统管理器（Local-first + 安全门禁 + 审计可追溯）`。
- 首个冻结范围：`架构 + API 契约 + 数据模型` 三件套先冻结，其余设计按冻结清单分批完成。

## 项目概述
Synora 是一个本地优先、AI 驱动的软件操作系统管理器，目标是将“软件发现、来源补链、下载校验、安装更新、风险控制、审计追溯”统一到一个可控流程中。

## 技术栈摘要（Draft）
- 语言与运行时：Rust（CLI-first，后续扩展桌面端）
- 数据存储：SQLite（本地审计与状态存储）
- 配置与仓库：JSON + YAML（`software.yaml`）
- 系统集成：Windows Registry / winget（MVP）
- AI 接入：Provider 抽象层（本地优先，可切换）

## 快速启动（实现阶段）
1. 运行 `cargo check`。
2. 初始化：`cargo run -- config init --json`。
3. 扫描入库：`cargo run -- software discover scan --json`。
4. 来源建议：`cargo run -- source suggest --json --limit 20`。
5. 完整回归：见 `docs/CLI_SMOKE_TESTS.md`。

## UI 起步（Phase 9）
仓库已提供最小 UI 原型：`ui/`（静态 HTML/CSS/JS）。
当前已接入 TailwindCSS（CDN 方式），无需额外前端构建步骤。

本地打开方式（任选其一）：
1. 直接打开 `ui/index.html`
2. 在仓库根目录执行 `python -m http.server 8080`，然后访问 `http://localhost:8080/ui/`

使用方式：
1. 在页面输入查询，生成 `ui search` 命令。
2. 将 `cargo run -- ui search --q "<query>" --json` 的输出粘贴到页面中。
3. 点击结果卡片，页面会生成对应 `ui action-run` 命令（高风险自动带 `--confirm`）。

## 当前可用命令矩阵
- `config`
1. `config init`
2. `config gate-show`
3. `config gate-set`
4. `config gate-history`
- `software`
1. `software discover scan`
2. `software list`
- `source`
1. `source suggest`
2. `source review`
3. `source review-bulk`
4. `source list`
5. `source apply-approved`
6. `source registry-list`
7. `source registry-disable`
8. `source registry-enable`
- `update`
1. `update check`
2. `update apply`（`--confirm` 需 `--execution-ticket`）
3. `update history`
- `ai`
1. `ai analyze`（plan-only）
2. `ai recommend`（plan-only）
3. `ai repair-plan`（plan-only，不触发真实变更）
- `ui`
1. `ui search`（聚合只读搜索入口）
2. `ui action-run`（高风险动作需 `--confirm`）
- `job`
1. `job submit`
2. `job list`
3. `job retry`
4. `job deadletter-list`
5. `job replay-deadletter`
- `cleanup`
1. `cleanup apply`（`--confirm` 需 `--execution-ticket`）
2. `cleanup history`

## 目录结构说明（当前）
- `docs/`：产品、架构、接口、数据模型与策略草案
- `decisions/`：ADR 与关键架构决策记录
- `logs/`：开发与冻结过程日志
- `src/`：实现代码（当前不作为本轮设计主焦点）

## 更新规则
- 当文档目录或阶段变化时必须同步更新。
- 仅记录已确认信息，不写推测性结论。
