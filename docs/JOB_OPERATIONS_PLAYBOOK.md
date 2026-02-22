# JOB_OPERATIONS_PLAYBOOK

## 文档目的
提供后台任务队列的日常运维与排障流程，降低任务失败后的处理成本。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：`job_queue` + `workers` + `scheduler`

## 日常操作清单（Draft）
1. 查看待处理任务
- `synora job list --status queued --limit 50 --json`

2. 查看失败任务
- `synora job list --status failed --limit 50 --json`

3. 查看死信任务
- `synora job list --status deadletter --limit 50 --json`

4. 查看任务详情
- `synora job show --id <job_id> --json --verbose`

5. 手动重试任务
- `synora job retry --id <job_id> --json`

## 标准排障流程（Draft）
1. 识别类型
- 判断任务属于发现、推荐、更新、清理或 AI 修复。

2. 读取错误
- 检查 `last_error`、`attempt_count`、`result_code`。

3. 分类处理
- `validation`：修正参数再重试
- `security`：修正 gate/confirm/权限后重试
- `integration`：检查外部依赖和网络，必要时延迟重试

4. 再次验证
- 执行 `job show` 确认状态流转正确。

5. 写审计
- 确保人工介入动作已记录到 `audit_event`。

## 常见问题速查（Draft）
1. 任务一直 queued
- 可能原因：worker 未运行、scheduler 配置错误
- 处理：检查 worker 进程与队列消费日志

2. 任务频繁 deadletter
- 可能原因：参数模板错误、外部依赖长期失败
- 处理：修复模板，必要时暂停 scheduler 模板任务

3. 高风险任务无法重试
- 可能原因：缺少 confirm 或 gate 未开启
- 处理：检查 `config gate-show` 与审批记录

## 运行健康指标（Draft）
1. 队列积压量：`queued_count`
2. 失败率：`failed_rate`
3. 死信率：`deadletter_rate`
4. 平均处理时长：`avg_latency_ms`

## 值班建议（Draft）
1. 每日检查 deadletter 队列
2. 每周复盘 top N 失败原因
3. 对重复失败任务建立固定修复脚本

## 更新规则
- 运维流程变更必须同步：
  - `docs/JOB_RETRY_POLICY_DRAFT.md`
  - `docs/API_SPEC.md`
  - `logs/DEVELOPMENT_LOG.md`
