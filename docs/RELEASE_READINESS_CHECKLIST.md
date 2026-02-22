# RELEASE_READINESS_CHECKLIST

## 目的
用于 Phase 8 / V1 发布前的统一门禁检查，确保功能、错误码、安全边界、审计可追溯全部满足 MVP DoD。

## 使用方式
- 执行环境：Windows PowerShell
- 仓库根目录：`C:\dev\synora`
- 推荐先执行：`scripts/smoke_phase8.ps1`

## A. 构建与回归
- [x] `cargo check` 通过
- [x] `cargo test` 通过
- [x] `scripts/smoke_phase8.ps1` 全流程通过

## B. 错误码契约
- [x] 成功路径返回 `0`
- [x] 参数校验错误返回 `2`
- [x] 安全阻断返回 `3`
- [x] 集成失败返回 `4`

## C. 安全门禁
- [x] `update apply --confirm` 必须要求 `--execution-ticket`
- [x] `cleanup apply --confirm` 必须要求 `--execution-ticket`
- [x] gate 关闭时真实变更路径被阻断（security=3）
- [x] `ui action-run` 高风险动作无 `--confirm` 被阻断（security=3）

## D. 审计可追溯
- [x] `update history` 含 `rollback_attempted/rollback_status/execution_ticket`
- [x] `cleanup history` 含 `rollback_attempted/rollback_status/execution_ticket`
- [x] `download history` 含结构化校验字段与失败统计
- [x] `ui_action_history` 有执行/阻断记录
- [x] `ai_repair_plan_history` / `ai_analyze_history` / `ai_recommend_history` 有记录

## E. 核心能力覆盖
- [x] Discovery：`software discover scan/history` 正常
- [x] Source：`suggest/list/review/review-bulk/apply-approved/registry-*` 正常
- [x] Update：`check/apply/history` 正常
- [x] Cleanup：`apply/history` 正常
- [x] Download：`start/list/show/retry/verify/history` 正常
- [x] Job Queue：`submit/list/retry/deadletter-list/replay-deadletter` 正常（含 deadletter 转移规则）
- [x] AI：`analyze/recommend/repair-plan` 均为 plan-only
- [x] UI：`search/action-run` 正常

## F. 发布判定
- [x] 关键命令成功率与预期退出码一致率 >= 99%
- [x] 高风险误执行次数 = 0
- [x] 审计查询覆盖率 = 100%

发布结论：
- [x] 通过（可进入 V1 发布）
- [ ] 不通过（阻断发布，需回归修复）
