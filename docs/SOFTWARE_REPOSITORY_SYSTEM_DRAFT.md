# SOFTWARE_REPOSITORY_SYSTEM_DRAFT

## 文档目的
定义 Synora 软件仓库系统（公共仓库 + 个人仓库 + AI）的边界、数据格式与演进路线。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 对标参考：Homebrew / AUR / Winget（能力借鉴，不复制其治理模式）

## 目标（Draft）
1. 提供统一的软件元数据来源层，减少手工维护下载链接。
2. 支持个人仓库与公共仓库并存，满足本地治理与社区贡献。
3. 使用 AI 辅助生成与修正仓库条目，但保留人工审核与安全门禁。

## 仓库分层（Draft）
1. `public`（公共仓库）
- 默认只读消费
- 由受信维护者发布
- 支持版本化索引

2. `personal`（个人仓库）
- 用户本地可写
- 支持导入/导出 `software.yaml`
- 可覆盖公共仓库条目（本地优先）

3. `candidate`（候选池）
- AI 生成或用户提交的待审条目
- 审核通过后可进入 `personal` 或 `public`

## 条目格式：`software.yaml`（Draft）
最小必填字段：
1. `name`
2. `version`
3. `install.url`
4. `check_update.provider`

建议字段：
1. `publisher`
2. `homepage`
3. `license`
4. `hash.sha256`
5. `uninstall.command`
6. `risk_level`
7. `tags`

示例：
```yaml
name: OBS
version: "30.0.0"
publisher: OBS Project
homepage: https://obsproject.com
license: GPL-2.0-or-later
risk_level: medium
tags:
  - streaming
  - video
install:
  url: https://cdn-fastly.obsproject.com/downloads/OBS-Studio-30.0.0-Windows-Installer.exe
  hash:
    sha256: "3e8f0b3f4c2a9d9f5d6f3d7a0c4a1b2e8f9c7d6a5b4c3d2e1f0a9b8c7d6e5f4"
uninstall:
  command: "\"C:\\Program Files\\obs-studio\\uninstall.exe\" /S"
check_update:
  provider: github_release
  repo: obsproject/obs-studio
```

## AI 参与点（Draft）
1. 生成候选条目：根据软件名和本地证据补全 `software.yaml`。
2. 质量评分：检测字段缺失、URL 可达性、风险信号。
3. 冲突提示：识别同名不同发布者、旧版本覆盖、哈希冲突。

约束：
1. AI 仅可写入 `candidate`，不可直接写入 `public`。
2. 高风险条目（来源不明、哈希缺失）必须人工确认。

## 仓库优先级（Draft）
1. `personal` > `public_trusted` > `candidate_pending`
2. 同名冲突时优先：
- 发布者一致 + 版本更高
- 哈希完整且来源受信
- 最近审核通过时间更新

## 安全边界（Draft）
1. 仓库来源必须有信任状态（trusted/untrusted/blocked）。
2. 导入的 `software.yaml` 必须做 schema 校验与路径/URL 安全校验。
3. 公共仓库提交流程必须可审计（提交人、时间、审核结论）。

## 分阶段策略（Draft）
1. MVP
- 个人仓库可写
- 公共仓库只读
- AI 只生成候选条目

2. Phase 2
- 社区提交流程（submit/review/publish）
- 仓库条目评分与自动告警

3. Phase 3
- 细粒度仓库权限模型（组织级）
- 插件化仓库适配器（GitHub/自建镜像）

## 仓库决策（Phase 1 Freeze）
1. 公共仓库首版策略：采用“单一官方受信源”。
2. `software.yaml` 版本策略：MVP 引入 `schema_version` 字段（默认 `v1`）。
3. 社区审核策略：Phase 2 采用双人审核（提交人之外至少 2 名审核通过）。

## 更新规则
- 仓库系统设计变更必须同步：
  - `docs/PRODUCT_SPEC.md`
  - `docs/ARCHITECTURE.md`
  - `docs/API_SPEC.md`
  - `docs/DATA_MODEL.md`
  - `docs/ROADMAP.md`
