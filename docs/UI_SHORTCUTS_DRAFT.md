# UI_SHORTCUTS_DRAFT

## 文档目的
定义全局搜索与核心工作流的快捷键策略，确保高频操作稳定、高风险操作可控、跨平台一致。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：CLI-first 阶段的全局搜索与后续 Desktop/Web UI

## 设计目标（Draft）
1. 高频动作一键可达，减少鼠标依赖。
2. 风险动作必须有显式确认，不提供危险“误触即执行”组合键。
3. 保持 Windows/macOS 的键位语义对齐（Ctrl 对应 Cmd）。

## 全局保留键（Draft）
1. 打开全局搜索：`Ctrl+K` / `Cmd+K`
2. 关闭/返回：`Esc`
3. 帮助面板：`Ctrl+/` / `Cmd+/`
4. 刷新结果：`Ctrl+R` / `Cmd+R`

## 搜索面板内快捷键（Draft）
1. 上下移动：`↑` / `↓`
2. 翻页：`PageUp` / `PageDown`
3. 执行主动作：`Enter`
4. 打开详情：`Tab`
5. 反向焦点：`Shift+Tab`
6. 打开上下文动作：`Ctrl+Enter` / `Cmd+Enter`
7. 复制命令：`Ctrl+Shift+C` / `Cmd+Shift+C`

## 结果分组跳转键（Draft）
1. 跳到 `Actions`：`Alt+1`
2. 跳到 `Software`：`Alt+2`
3. 跳到 `Downloads`：`Alt+3`
4. 跳到 `Jobs`：`Alt+4`
5. 跳到 `Security`：`Alt+5`
6. 跳到 `AI Suggestions`：`Alt+6`

## 任务与下载快捷动作（Draft）
1. 任务重试：`Ctrl+Shift+R` / `Cmd+Shift+R`
2. 打开任务详情：`Ctrl+J` / `Cmd+J`
3. 打开下载列表：`Ctrl+D` / `Cmd+D`
4. 打开安全面板：`Ctrl+Shift+S` / `Cmd+Shift+S`

## 安全与确认规则（Draft）
1. 高风险动作不允许绑定“单键执行”。
2. 涉及真实变更的快捷执行必须进入确认态（confirm step）。
3. 当 gate 为 disabled 时，快捷键仅允许查看提示，不触发执行。

## 冲突与优先级策略（Draft）
1. 系统保留键优先（OS-level shortcuts）。
2. Synora 全局保留键优先于页面级快捷键。
3. 页面级冲突时，以“更低风险动作”优先保留。
4. 冲突键必须在帮助面板显示替代键位。

## 可配置策略（Phase 2 Draft）
1. 允许用户重绑非高风险快捷键。
2. 支持按工作场景切换快捷键方案（开发/运维/安全）。
3. 支持插件声明快捷键（需权限与冲突校验）。

## 快捷键决策（Phase 1 Freeze）
1. 自定义快捷键：MVP 不开放，采用固定键位集。
2. Vim 导航：MVP 不引入 `j/k` 导航，保持统一方向键交互。
3. 双击 `Ctrl`：MVP 不支持双击 `Ctrl` 呼出，固定 `Ctrl/Cmd+K`。

## 更新规则
- 快捷键变更必须同步：
  - `docs/UI_GLOBAL_SEARCH_DRAFT.md`
  - `docs/UI_COMPONENT_SPEC_DRAFT.md`
  - `docs/UI_INTERACTION_COPY_DRAFT.md`
  - `docs/API_SPEC.md`
