# SECURITY

## 文档目的
定义项目安全边界、风险策略与处理流程。

## 当前状态
- 状态：v1 Frozen（安全策略冻结版）
- 安全模型：默认安全 + 门禁执行 + 审计追踪
- 安全基线草案：`docs/SECURITY_BASELINE_DRAFT.md`

## 上下文输入
- 项目类型：系统级操作工具
- 高风险特征：可能涉及系统变更

## 预期输出
- 明确默认安全策略
- 明确漏洞上报与响应流程

## 安全决策（Phase 1 Freeze）
1. 真实变更门禁策略：
- 默认 `gate=disabled`。
- 真实变更必须满足 `confirm + gate + approval_record + ticket`。

2. 风险分级标准：
- `low`：可快速回滚，影响面小。
- `medium`：存在状态变更，需显式确认。
- `high`：可能影响系统稳定，必须额外门禁校验。

3. 安全签署流程：
- 设计冻结后执行安全签署检查单。
- 高风险链路变更要求双人复核（你 + 我）并记录审计依据。

4. 插件签名与信任模型：
- 插件信任状态固定为 `trusted/untrusted/blocked`。
- V2 开始要求签名校验后才可进入 `trusted`。

## 安全正式策略（Phase 1 Freeze）
1. 安全原则
- 默认拒绝高风险真实变更；显式放行优先于隐式放行。
- 高风险操作必须满足 `confirm + gate + approval_record + ticket`。
- 关键链路（发现、下载、执行、回滚）必须可审计、可追溯、可复盘。

2. 威胁模型基线
- 重点威胁：路径遍历、未授权真实变更、插件越权、下载源投毒、仓库污染。
- 每条威胁必须对应至少一条阻断控制（验证、门禁、沙箱、签名、审计）。

3. 漏洞处理流程
- 分级：`critical / high / medium / low`。
- 时效：`critical` 24 小时内响应，`high` 72 小时内给出处置方案。
- 关闭条件：修复 + 回归测试 + 审计说明三项齐全。

4. 发布安全门禁
- 默认 `gate=disabled`，未通过安全门禁不得发布可执行真实变更版本。
- 发布前必须通过安全测试集合与关键路径 smoke。
- 发布记录必须包含审批与风险说明引用。

5. 插件安全边界
- `untrusted/blocked` 插件禁止进入执行链路。
- 插件执行必须通过 Security Guard，不得旁路调用高风险能力。
- 插件 action 必须命中 manifest 白名单，未声明即拒绝。

6. 插件权限策略
- 权限命名统一：`<domain>.<resource>.<verb>`。
- 默认最小权限，未声明权限一律拒绝。
- 高风险权限默认禁用，启用需 `confirm + gate + 审计理由`。

7. 插件供应链策略
- 生产态要求签名校验与来源信任校验。
- `api_compat` 不匹配拒绝加载。
- 连续失败超阈值自动转 `blocked`，恢复需审批。

8. AI 管理助手安全边界
- `analyze/recommend/repair-plan` 仅输出建议，不直接变更系统。
- `repair-apply` 仅在后续阶段开放，且必须走 gate 与审计。
- AI 输出必须携带 `reason`、`confidence`、`risk_level` 字段。

9. 下载模块安全边界
- 非白名单来源默认高风险并触发额外校验。
- 哈希或签名校验失败必须阻断后续执行。
- 下载路径必须通过路径遍历防护并写入审计。
- 详细策略以 `docs/DOWNLOAD_SOURCE_POLICY_DRAFT.md` 与 `docs/HASH_AND_SIGNATURE_POLICY_DRAFT.md` 为准。

10. Sandbox 执行边界
- 高风险执行必须进入受限上下文（最小权限、路径白名单、超时限制）。
- 沙箱策略违规时应直接返回 security blocked，不进入补偿执行。
- 详细策略以 `docs/SANDBOX_EXECUTION_POLICY_DRAFT.md` 为准。

11. 仓库系统安全边界
- 公共仓库默认只读，写入必须经过审核与审计。
- `software.yaml` 导入必须通过 schema 与 URL 安全校验。
- 非 trusted 仓库只能浏览，不能进入执行链路。
- AI 生成条目仅进入候选池，需人工审核后才能发布。

## 更新规则
- 任何安全相关实现变更后必须更新。
- 安全条目应与 ADR 和测试策略一致。
