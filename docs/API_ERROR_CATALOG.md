# API_ERROR_CATALOG

## 文档目的
统一 Synora CLI/API 的错误类型、错误码、典型消息与处理建议。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 作用范围：`api` / `services` / `workers` / `plugins` / `ai`

## 全局错误码（Draft）

| code | type | 含义 |
|---|---|---|
| 0 | success | 成功 |
| 2 | validation | 参数或用法错误 |
| 3 | security | 安全策略阻断 |
| 4 | integration | 外部依赖或运行时失败 |
| 10 | partial_success | 部分成功 |

## 错误目录（Draft）

### 1) 通用校验错误
- `validation.missing_required`
  - code: `2`
  - message: `required argument is missing`
  - hint: 补齐必填参数

- `validation.invalid_enum`
  - code: `2`
  - message: `invalid enum value`
  - hint: 使用文档中的合法枚举值

### 2) 安全门禁错误
- `security.gate_disabled`
  - code: `3`
  - message: `real mutation is disabled`
  - hint: 仅在审批后开启 gate

- `security.high_risk_without_confirm`
  - code: `3`
  - message: `risk 'high' requires explicit --confirm`
  - hint: 使用 `--confirm` 并检查审批链路

- `security.permission_denied`
  - code: `3`
  - message: `permission is not granted`
  - hint: 检查插件权限声明与信任状态

### 3) 集成错误
- `integration.provider_unavailable`
  - code: `4`
  - message: `provider is unavailable`
  - hint: 检查网络或外部依赖

- `integration.runtime_failed`
  - code: `4`
  - message: `plugin runtime failed to execute action`
  - hint: 查看 `error_message` 与插件日志

### 4) 队列错误
- `job.invalid_type`
  - code: `2`
  - message: `unknown job_type`
  - hint: 使用 `docs/JOB_TYPES_DRAFT.md` 中定义的类型

- `job.retry_not_allowed`
  - code: `2`
  - message: `job status does not allow manual retry`
  - hint: 仅对 `failed` / `deadletter` 执行 retry

- `job.security_blocked`
  - code: `3`
  - message: `job blocked by security policy`
  - hint: 修复 gate/confirm/权限后再重试

### 5) AI 错误
- `ai.invalid_goal`
  - code: `2`
  - message: `goal is required`
  - hint: 提供明确场景目标

- `ai.repair_apply_forbidden_in_mvp`
  - code: `3`
  - message: `repair-apply is not enabled in MVP`
  - hint: 使用 `repair-plan`

### 6) 下载错误
- `download.source_not_allowlisted`
  - code: `3`
  - message: `source domain is not in allowlist`
  - hint: 使用受信来源或人工确认高风险策略

- `download.checksum_failed`
  - code: `3`
  - message: `artifact checksum verification failed`
  - hint: 重新下载并校验来源/哈希

- `download.signature_invalid`
  - code: `3`
  - message: `artifact signature verification failed`
  - hint: 检查发布者与证书链

- `download.signature_missing`
  - code: `3`
  - message: `artifact signature is missing`
  - hint: 使用受信发布源或人工复核

### 7) 仓库系统错误
- `repository.untrusted_source`
  - code: `3`
  - message: `repository source is not trusted`
  - hint: 将仓库标记为 trusted 或改用个人仓库

- `repository.manifest_invalid`
  - code: `2`
  - message: `software.yaml validation failed`
  - hint: 补齐必填字段并检查 URL/版本格式

- `repository.public_write_disabled`
  - code: `3`
  - message: `public repository write is disabled in MVP`
  - hint: 使用 candidate 提交流程或个人仓库

## 错误响应建议结构（Draft）
```json
{
  "error": {
    "id": "security.gate_disabled",
    "type": "security",
    "code": 3,
    "message": "real mutation is disabled",
    "hint": "set execution.real_mutation_enabled=true after approval"
  }
}
```

## 更新规则
- 新增错误或改动错误语义必须同步：
  - `docs/API_SPEC.md`
  - `docs/JOB_RETRY_POLICY_DRAFT.md`
  - `SECURITY.md`
  - `logs/DEVELOPMENT_LOG.md`
