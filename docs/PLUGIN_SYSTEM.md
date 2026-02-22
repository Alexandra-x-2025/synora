# PLUGIN_SYSTEM

## 文档目的
定义 Synora 插件系统的目标、边界与最小可行架构，确保后续可扩展到多来源生态而不重构核心系统。

## 当前状态
- 状态：v0.1 Draft（继续设计中，未冻结）
- 定位：平台能力，不属于当前 MVP 强制交付

## 设计目标（Draft）
1. 让 Synora 通过插件扩展软件来源、更新规则、AI 工具与系统工具。
2. 核心系统保持稳定，新增能力通过插件接入。
3. 所有插件执行仍受统一安全门禁与审计链路约束。

## 插件能力范围（Draft）
1. `source_provider`：扩展软件来源发现与链接补全
- 示例：GitHub、JetBrains、Python(Pypi)、Node(npm)、Steam

2. `update_policy`：扩展更新策略与风险规则
- 示例：企业白名单策略、版本冻结策略

3. `ai_tool`：扩展推荐/解释能力
- 示例：来源评分器、风险解释器

4. `system_tool`：扩展系统治理能力
- 示例：清理器、健康检查器、修复脚本包装器

## 架构方案（Draft）
1. Phase 1（优先建议）
- 原生 Rust 插件（编译时或动态加载）
- 目录形态示例：`plugins/github.rs`, `plugins/steam.rs`
- 优点：实现快，调试成本低

2. Phase 2
- WASM 插件运行时（隔离执行）
- 目录形态示例：`plugins/github.wasm`
- 优点：隔离更强，未来多语言插件更可控

## 执行约束（Draft）
1. 插件不能绕过 Security Guard。
2. 高风险动作仍需 confirm + gate + approval record。
3. 插件输出必须结构化并写审计事件。
4. 默认禁用未签名或未信任插件。

## 最小接口草案（Draft）
```text
PluginManifest {
  id: string
  version: string
  kind: source_provider | update_policy | ai_tool | system_tool
  entry: string
  permissions: string[]
  signature: string?
}

PluginRuntime {
  list()
  enable(id)
  disable(id)
  run(id, action, payload)
}
```

## 1) Manifest 规范（Draft v0.1）

### 结构定义
```json
{
  "schema_version": "1.0",
  "plugin_id": "github.source",
  "name": "GitHub Source Provider",
  "version": "0.1.0",
  "kind": "source_provider",
  "entry": "plugins/github.wasm",
  "runtime": "wasm",
  "api_compat": ">=0.1,<0.2",
  "permissions": [
    "net.http.read",
    "inventory.read",
    "candidate.write"
  ],
  "actions": [
    "discover_sources",
    "enrich_candidate"
  ],
  "signature": "base64...",
  "publisher": "synora-official",
  "homepage": "https://example.com",
  "description": "Provide source links from GitHub releases"
}
```

### 字段规则
1. `plugin_id`
- 全局唯一，建议 `<vendor>.<name>`。

2. `kind`
- 枚举：`source_provider` | `update_policy` | `ai_tool` | `system_tool`。

3. `runtime`
- 枚举：`native` | `wasm`。

4. `api_compat`
- 声明可兼容的主程序 API 区间，不满足则拒绝加载。

5. `permissions`
- 最小权限声明，运行期按白名单校验。

6. `actions`
- 对外可调用动作名，`plugin run --action` 仅允许使用此列表。

7. `signature`
- 可空（开发态）；生产态建议强制非空。

## 2) 权限模型（Draft v0.1）

### 权限命名规范
- 采用 `<domain>.<resource>.<verb>`：
- `inventory.read`
- `candidate.write`
- `gate.read`
- `audit.write`
- `net.http.read`

### 最小权限矩阵（建议）
1. `source_provider`
- 推荐：`inventory.read` `candidate.write` `net.http.read`
- 禁止：`gate.write` `cleanup.execute`

2. `update_policy`
- 推荐：`inventory.read` `update.plan.write` `risk.evaluate`
- 禁止：`update.execute`（除非显式授权）

3. `ai_tool`
- 推荐：`inventory.read` `candidate.write`
- 禁止：`cleanup.execute` `gate.write`

4. `system_tool`
- 推荐：`inventory.read` `audit.write`
- 高风险权限：`cleanup.execute`（必须 confirm + gate）

### 执行策略
1. 权限最小化：未声明即拒绝。
2. 权限升级：需要管理员确认并记录 reason。
3. 高风险权限：默认禁用，需双重确认（confirm + gate）。

## 3) 生命周期与错误码映射（Draft v0.1）

### 生命周期状态
1. `discovered`
- manifest 被发现但未安装。

2. `installed`
- 已注册入库，默认 `enabled=false`。

3. `enabled`
- 可被调用，但每次执行仍做权限与信任校验。

4. `running`
- 当前 action 正在执行。

5. `disabled`
- 禁止执行。

6. `blocked`
- 因签名、权限、兼容性或策略问题封禁。

### 状态迁移
- `discovered -> installed -> enabled -> running -> enabled`
- `enabled -> disabled`
- `any -> blocked`
- `blocked -> disabled`（人工解除封禁后）

### 错误码映射（CLI）
1. `2` validation
- manifest 缺字段、参数错误、action 不存在。

2. `3` security
- 插件未信任、权限越权、需要 confirm/gate 但未满足。

3. `4` integration
- 插件运行时异常、依赖不可用、外部源调用失败。

4. `10` partial success
- 批量插件操作部分成功。

## 数据与审计（Draft）
1. 需要插件注册表（安装状态、签名状态、信任状态）。
2. 需要插件执行历史（谁、何时、执行了什么、结果如何）。
3. 需要插件权限声明与审批记录。

## 路线建议（Draft）
1. MVP：仅预留插件接口，不开放第三方插件安装。
2. Phase 2：开放内置官方插件集（GitHub/JetBrains/Python/Node）。
3. Phase 3：引入 WASM 沙箱与受控插件市场。

## 插件系统决策（Phase 1 Freeze）
1. 路线选择：V1/V2 先做 Rust 原生插件（可控交付），WASM 放到 Phase 3。
2. 签名与信任模型：V2 采用“签名 + 本地信任清单”模型，证书链扩展后置评估。
3. 权限粒度：采用细粒度 API 权限模型（`<domain>.<resource>.<verb>`）。
4. `signature` 字段策略：Phase 2 在生产模式设为强制字段。

## 更新规则
- 插件能力边界变更，必须同步：
  - `docs/PRODUCT_SPEC.md`
  - `docs/ARCHITECTURE.md`
  - `docs/DATA_MODEL.md`
  - `docs/API_SPEC.md`
  - `docs/ROADMAP.md`

## 关联文档
- `docs/PLUGIN_MANIFEST_SCHEMA.md`
- `docs/PLUGIN_PERMISSION_MATRIX.md`
- `docs/PLUGIN_LIFECYCLE_SEQUENCE.md`
