# PLUGIN_PERMISSION_MATRIX

## 文档目的
定义插件类型与权限边界的对应关系，作为安全审查与运行时授权基准。

## 当前状态
- 状态：v0.1 Draft（未冻结）

## 权限命名规范
- 格式：`<domain>.<resource>.<verb>`
- 示例：`inventory.read`、`candidate.write`、`cleanup.execute`

## 权限分级（Draft）
1. `P0` 低风险：只读权限
- 如：`inventory.read`、`gate.read`

2. `P1` 中风险：写入业务候选或策略
- 如：`candidate.write`、`update.plan.write`

3. `P2` 高风险：触发系统变更或清理执行
- 如：`update.execute`、`cleanup.execute`

## 插件类型权限矩阵（Draft）

| 插件类型 | 默认允许 | 条件允许 | 默认禁止 |
|---|---|---|---|
| `source_provider` | `inventory.read`, `candidate.write`, `net.http.read` | `audit.write` | `gate.write`, `cleanup.execute`, `update.execute` |
| `update_policy` | `inventory.read`, `risk.evaluate`, `update.plan.write` | `audit.write` | `cleanup.execute`, `gate.write` |
| `ai_tool` | `inventory.read`, `candidate.write` | `net.http.read` | `cleanup.execute`, `update.execute`, `gate.write` |
| `system_tool` | `inventory.read`, `audit.write` | `cleanup.execute`(需 confirm+gate) | `gate.write` |

## 授权规则（Draft）
1. 未声明权限即拒绝。
2. 请求超出 manifest 声明范围即拒绝并记审计。
3. `P2` 权限必须满足：
- 插件 `trust_status=trusted`
- 用户显式 `--confirm`
- `real_mutation_enabled=true`
- 审批记录可用

## 审计要求（Draft）
1. 每次授权决策都写 `audit_event`。
2. 每次插件执行写 `plugin_execution_history`，记录 `result_code`。
3. 拒绝事件必须包含：`plugin_id`、`action`、缺失权限。

## 权限决策（Phase 1 Freeze）
1. 权限粒度：引入按命令/API 粒度权限（细于 action 层）。
2. 临时授权：支持带过期时间的临时授权 token（仅用于中风险权限）。
3. 企业模式：增加组织策略覆盖层，组织策略优先于本地策略。

## 更新规则
- 权限策略改动必须同步：
  - `SECURITY.md`
  - `docs/PLUGIN_SYSTEM.md`
  - `docs/API_SPEC.md`
