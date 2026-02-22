# CARGO_DEPENDENCY_BASELINE_DRAFT

## 文档目的
将 `docs/TECH_STACK.md` 的技术选型映射为可执行的 `Cargo.toml` 依赖清单草案。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 适用阶段：V1 设计到实现过渡

## 设计原则
1. 最小可用：先满足 V1 核心链路，不提前引入重依赖。
2. 安全优先：涉及执行、下载、签名校验的依赖独立评审。
3. 分层清晰：依赖按模块归类，避免“全局工具箱”式膨胀。

## 依赖基线（V1 Draft）
1. CLI 与配置
- `clap`：命令解析
- `serde`：序列化/反序列化
- `serde_json`：JSON 输出
- `serde_yaml`：`software.yaml` 解析
- `thiserror`：错误类型定义

2. 存储与数据
- `rusqlite`（建议启用 `bundled` feature）：SQLite 访问
- `chrono`（可选）：时间格式化展示（若仅 Unix ts 可暂缓）

3. 下载与校验
- `reqwest`：HTTP/HTTPS 下载
- `sha2`：SHA256 校验
- `hex`：摘要编码处理
- `url`：URL 合法性校验

4. Windows 集成
- `windows`：Registry/证书等 WinAPI 封装

5. 运行与并发（按实现策略二选一）
- 方案 A（推荐）：`tokio`（异步 worker + 定时任务）
- 方案 B（简化）：标准线程 + 阻塞 I/O（先不引入 `tokio`）

6. 日志与可观测
- `tracing`（可选，建议 V1.1）
- `tracing-subscriber`（可选，建议 V1.1）

## 暂缓依赖（Phase 2+）
1. 插件 WASM
- `wasmtime`（Phase 3 评估）

2. 分布式队列
- Redis/NATS 客户端（Phase 3）

3. 向量/语义检索
- 向量数据库相关 SDK（暂缓）

## 建议依赖分组模板（Draft）
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
thiserror = "1"
rusqlite = { version = "0.32", features = ["bundled"] }
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
sha2 = "0.10"
hex = "0.4"
url = "2"
windows = "0.58"
# tokio = { version = "1", features = ["rt-multi-thread", "macros", "time"] }
# tracing = "0.1"
# tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter"] }
```

说明：
- 版本号当前为“草案范围”，实现前按当时稳定版本校准。
- `tokio`、`tracing` 先注释，避免在未定执行模型时过早绑定。

## 与当前 `Cargo.toml` 差异
当前 `Cargo.toml` 仅有空 `[dependencies]`。
目标是按本清单分两步引入：
1. Step 1：CLI + 存储 + YAML/JSON（不含并发框架）
2. Step 2：下载与 Windows 集成

## 实施顺序（建议）
1. 引入 `clap` / `serde` / `serde_json` / `thiserror`
2. 引入 `rusqlite` 并打通最小读写
3. 引入 `serde_yaml` 打通 `software.yaml` 解析
4. 引入 `reqwest` / `sha2` / `url` 落地下载与校验
5. 最后决定 `tokio` 与 `tracing`

## 依赖决策（Phase 1 Freeze）
1. Worker 执行模型：V1 采用 `tokio`（统一异步任务与调度模型）。
2. `windows` crate 覆盖范围：V1 覆盖 Registry + 基础签名校验入口。
3. `rusqlite` 策略：V1/V2 固定 `bundled`，Phase 3 评估系统库切换。

## 更新规则
- 本清单变更必须同步：
  - `docs/TECH_STACK.md`
  - `Cargo.toml`
  - `logs/DEVELOPMENT_LOG.md`
