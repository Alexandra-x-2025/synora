# UI_COMPONENT_SPEC_DRAFT

## 文档目的
定义全局搜索 UI 的核心组件、状态与数据契约，作为后续前端实现标准。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：Global Search / Command Palette

## 组件清单（Draft）

### 1) `SearchPalette`
职责：
- 承载搜索输入、结果分组、快捷操作

输入：
- `query: string`
- `open: boolean`
- `loading: boolean`
- `groups: SearchGroup[]`

输出事件：
- `onQueryChange(query)`
- `onSelect(item)`
- `onClose()`

### 2) `SearchInput`
职责：
- 输入关键词/自然语言
- 展示即时提示（历史/建议）

状态：
- `idle` / `typing` / `submitting`

### 3) `ResultGroup`
职责：
- 分组展示结果

分组类型：
- `actions` / `software` / `downloads` / `jobs` / `security` / `ai_suggestions`

### 4) `ResultItem`
职责：
- 展示单条结果并响应选择

字段：
- `title`
- `subtitle`
- `type`
- `risk_level`
- `confidence`
- `shortcut_hint`
- `action_id`

### 5) `RiskBadge`
职责：
- 显示风险等级：`low` / `medium` / `high`

### 6) `ConfirmDialog`
职责：
- 高风险动作二次确认

输入：
- `action_title`
- `risk_level`
- `impact_summary`
- `rollback_hint`

输出事件：
- `onConfirm()`
- `onCancel()`

### 7) `ExecutionToast`
职责：
- 展示执行反馈（success / blocked / failed）

## 关键状态机（Draft）

1. 面板状态
- `closed -> opening -> open -> closing -> closed`

2. 查询状态
- `idle -> searching -> result_ready`
- `searching -> error`

3. 动作执行状态
- `selected -> confirming -> executing -> (success | blocked | failed)`

## 数据结构（Draft）

```ts
type SearchGroup = {
  type: "actions" | "software" | "downloads" | "jobs" | "security" | "ai_suggestions";
  items: SearchItem[];
};

type SearchItem = {
  id: string;
  title: string;
  subtitle?: string;
  type: string;
  risk_level?: "low" | "medium" | "high";
  confidence?: number;
  shortcut_hint?: string;
  action_id: string;
};
```

## 性能目标（Draft）
1. 打开面板首帧 < 100ms
2. 查询响应（本地数据）< 120ms
3. 结果分组渲染 < 16ms（单帧）

## 可访问性（Draft）
1. 全键盘可操作
2. 焦点可见与可追踪
3. 结果项支持屏幕阅读器语义

## 更新规则
- 组件结构变更必须同步：
  - `docs/UI_GLOBAL_SEARCH_DRAFT.md`
  - `docs/UI_MAP.md`
  - `docs/UX_FLOW.md`
