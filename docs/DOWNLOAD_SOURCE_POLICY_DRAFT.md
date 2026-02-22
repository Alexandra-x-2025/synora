# DOWNLOAD_SOURCE_POLICY_DRAFT

## 文档目的
定义下载来源的信任策略、风险分级与拦截规则，确保下载模块在安全边界内运行。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：`download.start` / `download.verify` / `download.fetch`

## 来源信任模型（Draft）
1. `trusted`
- 官方域名、已审批来源、签名与校验一致

2. `untrusted`
- 未在白名单中的来源，默认高风险

3. `unknown`
- 无法确定来源可信度，需人工复核

## 白名单策略（Draft）
1. 白名单单位：域名（可扩展到 path 级）
2. 白名单来源：
- 核心配置文件（手工维护）
- 审批记录同步项（后续扩展）
3. 匹配规则：
- 精确匹配优先
- 子域名是否继承为可配置项（默认不继承）

## 风险分级（Draft）
1. 低风险
- 白名单域名 + 哈希校验通过

2. 中风险
- 白名单域名但哈希缺失

3. 高风险
- 非白名单域名
- 哈希校验失败
- 重定向到未知域名

## 拦截规则（Draft）
1. 高风险下载默认阻断，返回 `security`。
2. 校验失败强制阻断后续安装链路。
3. `allow_untrusted_source=true` 仅在显式确认后允许（后续受 gate 约束）。

## 重定向策略（Draft）
1. 最大重定向次数：5
2. 重定向跨域时重新执行白名单校验
3. 出现协议降级（https -> http）直接阻断

## 审计要求（Draft）
每次下载与校验必须记录：
1. `source_url`
2. `resolved_domain`
3. `trust_level`
4. `checksum_expected` / `checksum_actual`
5. 结果与阻断原因

## 来源策略决策（Phase 1 Freeze）
1. 企业策略覆盖：允许企业策略覆盖本地白名单（企业模式优先级更高）。
2. 子域继承默认值：默认关闭（仅精确域名匹配）。
3. 非白名单临时授权：允许一次性临时授权，但必须显式确认并写审计。

## 更新规则
- 策略变更必须同步：
  - `SECURITY.md`
  - `docs/API_SPEC.md`
  - `docs/JOB_TYPES_DRAFT.md`
  - `docs/API_ERROR_CATALOG.md`
