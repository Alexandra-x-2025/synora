# HASH_AND_SIGNATURE_POLICY_DRAFT

## 文档目的
定义下载产物的哈希与签名联合校验策略，形成“来源可信 + 内容完整 + 发布者可信”的三重防线。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：`download.fetch` / `download.verify` / 安装前校验

## 策略分层（Draft）

### 1) 哈希校验（MVP 必做）
1. 算法：`SHA256`
2. 输入：期望哈希（来源候选/清单/人工提供）
3. 输出：`verify_success` / `verify_failed`
4. 失败策略：阻断安装并写审计

### 2) 签名校验（Phase 2）
1. 优先目标：Windows 可执行文件签名（Authenticode）
2. 校验项：签名存在、证书链有效、发布者匹配
3. 失败策略：高风险标记 + 阻断执行

### 3) 来源一致性校验
1. 记录来源域名与最终下载域名
2. 若跨域重定向，必须重新评估信任等级
3. 来源不一致时默认升高风险等级

## 判定矩阵（Draft）

| hash | signature | source trust | 结果 |
|---|---|---|---|
| pass | pass | trusted | allow |
| pass | missing | trusted | warn (MVP allow, Phase2 configurable) |
| fail | any | any | block |
| pass | fail | any | block |
| pass | pass | untrusted | block_or_manual_review |

## 审计字段要求（Draft）
1. `checksum_expected`
2. `checksum_actual`
3. `signature_status`
4. `publisher`
5. `trust_level`
6. `decision`

## 错误映射建议（Draft）
1. `download.checksum_failed` -> code `3`
2. `download.signature_invalid` -> code `3`
3. `download.signature_missing` -> code `3`（Phase 2 可配置为 warn）

## 策略决策（Phase 1 Freeze）
1. Phase 2 签名缺失默认策略：`block`（默认阻断执行）。
2. 发布者白名单策略：项目级默认 + 组织级覆盖（企业模式）。
3. 多算法策略：Phase 1 固定 `SHA256`；`SHA512` 作为 Phase 2 增强。

## 更新规则
- 策略变更必须同步：
  - `docs/DOWNLOAD_SOURCE_POLICY_DRAFT.md`
  - `SECURITY.md`
  - `docs/API_ERROR_CATALOG.md`
  - `docs/API_SPEC.md`
