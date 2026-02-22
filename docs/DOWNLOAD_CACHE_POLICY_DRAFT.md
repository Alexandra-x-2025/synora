# DOWNLOAD_CACHE_POLICY_DRAFT

## 文档目的
定义下载产物缓存的保留、复用与清理策略，平衡磁盘占用与可追溯性。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用范围：`download_artifact` / `download.cleanup`

## 缓存目标（Draft）
1. 避免重复下载，提升任务效率。
2. 保留足够证据满足审计需求。
3. 控制长期磁盘占用。

## 缓存键策略（Draft）
1. 主键建议：`(file_path, sha256)`（已在 SQL 草案中体现）
2. 复用判断：
- `sha256` 相同且文件存在 -> 可复用
- 文件缺失或哈希不一致 -> 视为失效

## 保留策略（Draft）
1. 默认保留期：30 天
2. 活跃产物（近期使用）可刷新 `expires_at`
3. 高风险来源产物可缩短保留期（如 7 天）

## 清理策略（Draft）
1. 清理任务：`download.cleanup`
2. 默认仅清理“过期且未引用”产物
3. 清理前执行二次校验：
- 是否被当前任务引用
- 是否存在未完成安装流程引用

## 引用追踪（Draft）
1. 短期方案：通过 `software_id + created_at` 近似追踪
2. 后续方案：增加显式 `artifact_reference` 表

## 失败与恢复（Draft）
1. 清理失败写入 `audit_event`，保留错误信息
2. 删除失败不影响主任务流程
3. 缓存目录损坏时可触发全量重建任务（后续）

## 审计要求（Draft）
清理动作必须记录：
1. 清理数量
2. 释放空间估算
3. 清理策略参数（days / referenced-only）
4. 失败明细

## 缓存策略决策（Phase 1 Freeze）
1. 默认保留期策略：按来源风险动态调整（低风险 30 天；高风险 7 天）。
2. 手动固定策略：允许用户标记产物为“固定保留”，清理任务默认跳过。
3. CLI 能力：增加 `download cache stats` 命令（V1 范围内实现）。

## 更新规则
- 策略变更必须同步：
  - `docs/DATABASE_DESIGN_DRAFT.md`
  - `docs/DATA_MODEL.md`
  - `docs/JOB_TYPES_DRAFT.md`
  - `docs/API_SPEC.md`
