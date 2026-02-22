# Synora -- Real Mutation Gate Strategy (Draft v0.1)

Date: 2026-02-22
Status: Draft for approval
Scope: Control strategy for enabling real cleanup mutation

---

## 1. Objective / 目标

English:
- Define a deterministic and auditable switch for real mutation.
- Ensure simulation remains default-safe.
- Prevent accidental production mutation enablement.

中文:
- 定义可确定、可审计的真实变更开关策略。
- 保持 simulation 为默认安全模式。
- 防止误启用真实生产变更。

---

## 2. Recommended Decision / 推荐决策

Decision:
- Use a runtime gate as primary control.
- Require signed approval record as policy precondition.
- Keep build profile default as simulation-only.

Rationale:
- Runtime gate gives operational flexibility.
- Approval artifact enforces governance and accountability.
- Default simulation reduces accidental blast radius.

---

## 3. Gate Model / 门禁模型

Three-layer gate:
1. Policy gate:
- Security sign-off checklist approved.
- Approval record completed.
2. Runtime gate:
- Explicit config value enables real mutation.
- Default is disabled.
3. Command gate:
- `--confirm` required.
- `--risk high|critical` must pass explicit confirmation policy.

Effective behavior:
- Any gate not satisfied -> block real mutation and return security/validation error.

---

## 4. Proposed Configuration Contract / 配置契约建议

Config key proposal:
- `execution.real_mutation_enabled: false` (default)
- `execution.gate_version: "phase3-draft-v1"`
- `execution.approval_record_ref: "<doc-or-pr-link>"`

Rules:
- Real mutation requires `real_mutation_enabled=true`.
- `approval_record_ref` must be non-empty when enabling real mutation.
- Gate version must be written into audit records.

---

## 5. Audit and Traceability / 审计与可追溯

For each confirmed mutation request:
- Record gate state snapshot in audit metadata:
- gate enabled/disabled
- gate version
- approval record reference
- operator timestamp

Minimum trace requirement:
- Reviewer can reconstruct why mutation was allowed at that moment.

---

## 6. Rollout Plan / 推进计划

Step 1:
- Approve this gate strategy draft.
Step 2:
- Implement runtime config checks in cleanup execution path.
Step 3:
- Extend audit schema/metadata with gate snapshot fields.
Step 4:
- Add tests:
- gate disabled blocks mutation
- gate enabled without approval ref blocks mutation
- gate enabled with approval ref permits mutation path
Step 5:
- Controlled pilot rollout in isolated environment.

---

## 7. Open Items / 待决项

- Exact storage format for gate metadata (new columns vs JSON field).
- Whether gate toggle requires dual reviewer sign-off in enterprise mode.
- Whether gate change should require process restart.

