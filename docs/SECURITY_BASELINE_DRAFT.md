# SECURITY_BASELINE_DRAFT

## 文档目的
定义 Synora 当前阶段必须满足的最小安全基线，作为实现与评审的门槛标准。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：download / update / cleanup / plugin / ai repair

## 安全基线（Draft）

### 1) Hash 校验基线
1. 下载产物必须支持 SHA256 校验。
2. 校验失败直接阻断后续安装/执行链路。
3. 校验结果必须写审计。

### 2) Sandbox 执行基线
1. 高风险执行必须运行在受限执行上下文。
2. 受限上下文至少包含：最小权限、受控目录、超时限制。
3. Sandbox 异常必须可回滚或可恢复。

### 3) 来源信任基线
1. 非白名单来源默认高风险。
2. 重定向跨域必须重新执行信任校验。
3. 协议降级（https -> http）默认阻断。

### 4) 签名与发布者基线
1. MVP 至少完成 hash + 来源校验。
2. Phase 2 增加签名/发布者校验（Windows 优先）。
3. 签名校验失败按高风险处理并阻断执行。

### 5) 执行门禁基线
1. 高风险动作必须满足 `confirm + gate + approval_record + ticket`。
2. 任何绕过门禁的路径视为安全缺陷。
3. `repair-apply`、`update.apply`、`cleanup` 默认受门禁约束。

### 6) 审计追溯基线
1. 关键安全动作必须写结构化审计。
2. 审计至少包含：输入摘要、决策、结果、错误、操作者、时间。
3. 人工重试/人工放行必须可追溯。

## 基线验收清单（Draft）
1. Hash 校验失败用例存在并通过。
2. 高风险无 confirm 被阻断。
3. Gate 关闭时真实执行被阻断。
4. 非白名单下载来源被阻断。
5. 审计查询可检索上述安全事件。

## 基线决策（Phase 1 Freeze）
1. Sandbox 在 MVP 采用系统级受限执行（Job Object + 受限令牌），不采用纯模拟隔离。
2. Phase 2 签名校验最小覆盖范围：Windows `PE/EXE + MSI`。
3. 审计保留与归档：在线保留 180 天，超过后按月归档。

## 更新规则
- 基线变更必须同步：
  - `SECURITY.md`
  - `docs/ARCHITECTURE.md`
  - `docs/API_SPEC.md`
  - `docs/API_ERROR_CATALOG.md`
