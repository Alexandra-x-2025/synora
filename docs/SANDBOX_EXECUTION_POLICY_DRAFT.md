# SANDBOX_EXECUTION_POLICY_DRAFT

## 文档目的
定义高风险任务在受限执行环境中的策略，降低误操作与恶意输入风险。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：`update.apply` / `cleanup.quarantine` / `ai repair-apply`（Phase 2）

## Sandbox 目标（Draft）
1. 限制可访问资源范围（文件/进程/网络）。
2. 限制执行时长与资源消耗。
3. 将失败影响限制在可恢复边界内。

## 约束策略（Draft）
1. 权限最小化
- 默认只读系统信息权限
- 仅在必要操作时授予临时写权限

2. 路径限制
- 允许目录：受控工作目录与下载缓存目录
- 禁止路径：系统关键目录与用户私有目录（除显式授权）
- 必须执行路径遍历防护

3. 进程限制
- 禁止任意子进程拉起
- 仅允许白名单命令
- 单任务最大并发进程数受限

4. 网络限制
- 默认禁止任意外联
- 下载任务仅允许白名单域名
- 协议降级禁止

5. 资源限制
- 超时（示例）：300s
- 内存与文件大小阈值
- 超限任务强制终止并记录审计

## 失败与回滚（Draft）
1. 执行失败后进入标准状态：`failed` / `rollback_success` / `rollback_failed`
2. 回滚动作必须独立审计
3. Sandbox 终止视为集成失败并进入重试/死信策略

## 审计要求（Draft）
每次 sandbox 执行必须记录：
1. `policy_version`
2. `allowed_paths`
3. `command_profile`
4. `timeout_limit`
5. 最终状态与错误原因

## Sandbox 决策（Phase 1 Freeze）
1. Windows sandbox 技术选型：Phase 1 采用 Job Object + 受限令牌组合；AppContainer 后置评估。
2. 网络策略位置：Phase 1 不引入独立代理层，先在执行层内做白名单与协议控制。
3. 策略版本存储：记录在 `config.json.execution.sandbox_policy_version`，并随审计事件落库。

## 更新规则
- 策略变更必须同步：
  - `docs/SECURITY_BASELINE_DRAFT.md`
  - `SECURITY.md`
  - `docs/ARCHITECTURE.md`
