# UI_SEARCH_RANKING_DRAFT

## 文档目的
定义全局搜索结果排序策略与权重规则，保证结果稳定、可解释、可优化。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：Global Search（Raycast 风格入口）

## 排序目标（Draft）
1. 优先返回最可能执行的动作。
2. 在安全边界内提升效率，不牺牲风险可见性。
3. 为后续个性化排序保留扩展位。

## 基础评分模型（Draft）
总分：
`score = text_match + intent_match + recency + reliability + safety_adjust`

## 权重建议（Phase 1 Draft）
1. `text_match`（0-50）
- 精确命中：+50
- 前缀命中：+35
- 模糊命中：+20

2. `intent_match`（0-20）
- 与当前意图分类一致（下载/任务/安全/AI）：+20
- 弱相关：+10

3. `recency`（0-15）
- 最近常用动作加分
- 长期未使用衰减

4. `reliability`（0-10）
- 历史成功率高加分
- 历史失败/阻断多减分

5. `safety_adjust`（-20~+5）
- 高风险动作默认不置顶：-10~-20
- 低风险且可直接执行：+5

## 分组排序策略（Draft）
1. 先组内排序，再组间排序。
2. 默认组间顺序：
- `Actions` > `Software` > `Downloads` > `Jobs` > `Security` > `AI Suggestions`
3. 若用户关键词明显指向某组（如 `failed jobs`），该组可提升到首位。

## 安全优先规则（Draft）
1. 高风险动作不得因文本匹配过高直接置顶并自动执行。
2. 必须显示风险等级与确认提示。
3. 被安全阻断过多的动作增加惩罚分。

## 可解释性输出（Draft）
每条结果建议返回：
1. `score_total`
2. `score_breakdown`
3. `rank_reason`

## Phase 2 扩展（Draft）
1. 个性化权重学习（用户行为）
2. 语义检索增强（意图向量）
3. 组织策略覆盖（企业模式）

## 排序策略决策（Phase 1 Freeze）
1. 置顶策略：MVP 不支持用户手动固定首位，避免破坏安全排序约束。
2. 场景策略：MVP 不按工作区切换排序配置，采用单一全局策略。
3. 高风险惩罚：默认惩罚分固定为 `-15`，MVP 不开放配置。

## 更新规则
- 排序规则变更必须同步：
  - `docs/UI_GLOBAL_SEARCH_DRAFT.md`
  - `docs/UI_COMPONENT_SPEC_DRAFT.md`
  - `docs/UX_FLOW.md`
  - `docs/API_SPEC.md`
