# JOB_RETRY_POLICY_DRAFT

## 文档目的
定义后台任务失败后的重试、死信与人工介入策略，确保队列行为可预测且可审计。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：`job_queue` + `workers`

## 状态语义（Draft）
1. `queued`：等待执行
2. `running`：执行中
3. `success`：成功完成
4. `failed`：本次执行失败
5. `retrying`：进入重试等待状态
6. `deadletter`：超过重试阈值，等待人工处理

## 默认重试策略（Draft）
1. `max_attempts` 默认值：`3`
2. 重试触发：`failed` 且 `attempt_count < max_attempts`
3. 回退策略：指数退避（`30s`, `120s`, `300s`）
4. 达到上限：状态置为 `deadletter`

## 可重试任务类型建议（Draft）
1. 推荐自动重试
- `source.suggest.enrich`
- `update.check.batch`
- `ai.analyze`
- `ai.recommend`

2. 条件重试
- `discovery.scan.registry`（非权限类失败可重试）

3. 默认不自动重试
- `update.apply.package`
- `cleanup.quarantine`
- 原因：高风险任务需人工复核

## 人工介入流程（Draft）
1. 通过 `job list --status deadletter` 查找死信任务
2. 使用 `job show --id <job_id>` 查看失败原因
3. 修正参数或环境后执行 `job retry --id <job_id>`
4. 所有人工重试必须写 `audit_event`

## 错误分类（Draft）
1. 可重试错误（暂态）
- 网络超时
- 外部服务限流
- 临时资源不足

2. 不可重试错误（确定性）
- 参数非法
- 权限不足
- gate/security 拦截
- manifest/action 不匹配

## 安全规则（Draft）
1. `security` 拦截导致的失败，不得自动重试。
2. 高风险 job 的手动重试必须再次满足 confirm + gate。
3. 死信任务保留完整 `last_error` 与时间链路。

## 指标建议（Draft）
1. `retry_success_rate`
2. `deadletter_rate`
3. `avg_attempt_count`
4. `p95_job_latency`

## 重试策略决策（Phase 1 Freeze）
1. `max_attempts` 支持按 `job_type` 覆盖（默认 3，不同任务可单独定义）。
2. 引入 jitter（默认 `±20%`）避免重试风暴。
3. 支持批量死信回放，但仅允许非高风险任务自动回放；高风险任务需人工确认。

## 更新规则
- 重试策略变更必须同步：
  - `docs/JOB_TYPES_DRAFT.md`
  - `docs/API_SPEC.md`
  - `docs/DATA_MODEL.md`
  - `docs/ARCHITECTURE.md`
