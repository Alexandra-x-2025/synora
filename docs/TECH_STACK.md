# TECH_STACK

## 文档目的
记录 Synora 技术选型、选择理由与替代方案，支撑后续实现与评审。

## 当前状态
- 状态：v1 Frozen（技术基线冻结）
- 范围：CLI-first、本地优先、插件可扩展、仓库系统与 AI 安全编排

## 上下文输入
- 产品定位：AI 驱动的软件操作系统管理器
- 平台优先级：Windows first
- 安全约束：高风险操作必须经过 gate 与审批记录

## 预期输出
- 形成可审查技术栈清单
- 关键技术都有选型理由与备选项

## 选型清单（Draft）
1. 语言与运行时
- 主语言：Rust
- 理由：内存安全、性能稳定、CLI 生态成熟
- 备选：Go（开发速度快但与现有方向不一致）

2. CLI 框架
- 候选：`clap`
- 理由：参数声明与测试能力成熟

3. 持久化
- 主库：SQLite（本地）
- 配置：`config.json`
- 理由：本地优先、部署简单、审计查询成本低

4. 序列化与数据结构
- 候选：`serde` + `serde_json`
- 理由：JSON 契约稳定，便于 AI/脚本消费

5. Windows 集成
- 候选：Windows Registry API（发现）、winget（包管理集成）
- 理由：满足 MVP Registry-only Discovery 结论

6. 插件系统（Draft）
- Phase 2 首选：Rust 原生插件运行时
- Phase 3 评估：WASM 插件沙箱（`*.wasm`）
- 理由：先保证交付速度，再提升隔离与生态开放能力

7. 日志与审计
- 结构化审计写入 SQLite
- 导出格式：JSON

## 最终选型基线（V1 Draft）
1. Core 语言与工具链
- Rust stable（Edition 2021）
- Cargo（构建/测试/依赖管理）
- 选择理由：类型系统 + 所有权模型更适合系统级安全工具

2. CLI 与输出契约
- `clap`：命令与参数解析
- `serde` + `serde_json`：JSON 输出契约
- 选择理由：可测试、可扩展、对 AI/脚本友好

3. 存储层
- SQLite（本地单文件）
- `rusqlite`（SQLite 访问）
- `config.json`（门禁与本地配置）
- 选择理由：部署零依赖，满足 local-first 与审计查询

4. 系统集成（Windows）
- Registry 读取：Windows API 封装（MVP Registry-only）
- 包管理集成：winget（先集成读取/计划链路）
- 选择理由：与 Windows 生态一致，覆盖“发现 + 安装治理”核心需求

5. 后台任务
- 本地队列：SQLite 表实现（`job_queue` + `scheduler_task`）
- 调度：应用内 scheduler
- 选择理由：MVP 降低部署复杂度，保留向 Redis/NATS 迁移空间

6. 下载与校验
- HTTP 下载：`reqwest`（阻塞/异步根据 worker 形态选择）
- Hash：SHA256（`sha2`）
- 签名校验：Windows Authenticode / 证书链策略（先策略接口，后逐步实现）
- 选择理由：先满足“来源+完整性校验”，再增强签名生态兼容

7. 软件仓库系统
- 元数据格式：YAML（`software.yaml`）
- 解析：`serde_yaml`
- 仓库分层：`public`（只读）/`personal`（可写）/`candidate`（待审）
- 选择理由：人类可读、社区贡献成本低，适配 Homebrew 类工作流

8. AI 层
- 决策策略：规则优先 + AI 辅助（默认 plan-only）
- 输出契约：`reason` + `confidence` + `risk_level`
- 模型接入：先抽象 provider 接口（本地/远程可替换），不在 V1 绑定单一厂商
- 选择理由：先保证安全与可解释，再追求模型能力上限

9. 插件系统
- V1：插件接口与权限模型先行，不开放第三方安装
- V2：官方插件（Rust 原生）
- V3：评估 WASM（wasmtime 优先评估）
- 选择理由：先可控再开放，避免早期供应链风险

10. 可观测与日志
- 业务审计：SQLite 审计表（结构化）
- 运行日志：结构化文本（后续可接 tracing）
- 导出：JSON
- 选择理由：满足合规追溯与自动化分析

## 版本与依赖策略（Draft）
1. Rust 版本策略
- 跟随 stable，每月评估一次升级窗口
- 非安全关键依赖采用“小步升级”

2. Cargo 依赖策略
- 默认锁定次版本范围（`^`）
- 安全相关库优先补丁升级
- 重大升级需补 ADR 与回归测试记录

3. 平台支持策略
- V1：Windows 11/10（x64）为主
- 非 Windows 仅保证文档与编译可评估，不承诺功能可用

## 备选与暂缓
1. 暂缓：Postgres / Redis / NATS
- 原因：V1 local-first，不引入外部基础设施

2. 暂缓：Electron/Tauri GUI
- 原因：先把 CLI 契约与核心链路做稳

3. 暂缓：向量数据库
- 原因：AI 任务当前以规则与结构化数据为主，尚无必要

## 插件技术方向（Draft）
1. 插件目录
- 开发态示例：`plugins/github.rs`, `plugins/steam.rs`
- 运行态示例：`plugins/*.wasm`（Phase 3）

2. 插件类型
- `source_provider`
- `update_policy`
- `ai_tool`
- `system_tool`

3. 插件安全
- 必须声明权限
- 非 trusted 插件不可启用
- 所有插件执行写审计

## 技术决策（Phase 1 Freeze）
1. Rust 原生插件加载方式：V1/V2 采用编译期注册（安全优先），动态加载后置评估。
2. WASM 运行时：选 `wasmtime` 作为 Phase 3 默认评估目标。
3. 插件签名链路：V2 开始要求插件包签名 + 本地信任清单，V3 再评估证书链扩展。
4. AI provider 首选：本地优先，云模型仅在显式配置后启用。
5. 下载签名覆盖：Phase 2 首覆盖 `PE/EXE + MSI`，`MSIX` 作为后续增强。

## 冻结落实清单（Phase 1 Freeze）
1. `Cargo.toml` 依赖对齐基线
- 依赖基线以 `docs/CARGO_DEPENDENCY_BASELINE_DRAFT.md` 为准。
- 当前冻结结论：先保留最小可交付依赖集，按 Phase 计划分批引入并回归验证。

2. Windows API 封装方案（冻结）
- 统一通过 `integration` 层封装 Registry/签名/系统能力访问。
- 禁止在 `api/services` 层直接调用平台 API。

3. 插件运行时 PoC 结论（冻结）
- V1/V2 采用 Rust 原生受控插件模型。
- WASM 运行时（`wasmtime`）仅保留 Phase 3 评估入口，不进入 V1 交付面。

4. AI provider 抽象接口（冻结）
- AI provider 采用可替换抽象接口，默认本地优先。
- 错误模型统一映射到 `validation/security/integration` 三类语义与退出码体系。

## 更新规则
- 技术栈变更必须同步：
  - `docs/ARCHITECTURE.md`
  - `docs/API_SPEC.md`
  - `docs/PLUGIN_SYSTEM.md`
  - `docs/CARGO_DEPENDENCY_BASELINE_DRAFT.md`
  - `logs/DEVELOPMENT_LOG.md`
- 不接受“只写库名不写理由”的选型记录。
