# PLUGIN_LIFECYCLE_SEQUENCE

## 文档目的
定义插件生命周期状态流转与执行时序，减少实现阶段的歧义。

## 当前状态
- 状态：v0.1 Draft（未冻结）

## 生命周期状态（Draft）
1. `discovered`
2. `installed`
3. `enabled`
4. `running`
5. `disabled`
6. `blocked`

## 状态转换图（Draft）
```text
discovered -> installed -> enabled -> running -> enabled
enabled -> disabled
disabled -> enabled
any -> blocked
blocked -> disabled (manual review)
```

## 执行时序（成功路径，Draft）
```text
User -> CLI: plugin run --id X --action A
CLI -> Service: parsed request
Service -> PluginRuntime: load manifest(X)
PluginRuntime -> SecurityGuard: validate trust/compat/action/permissions
SecurityGuard -> Service: allowed
Service -> PluginRuntime: execute(A)
PluginRuntime -> Repository: write plugin_execution_history(status=running)
PluginRuntime -> Repository: write plugin_execution_history(status=success, code=0)
Service -> CLI: success response
```

## 执行时序（安全阻断路径，Draft）
```text
User -> CLI: plugin run --id X --action A
CLI -> Service: parsed request
Service -> PluginRuntime: load manifest(X)
PluginRuntime -> SecurityGuard: validate
SecurityGuard -> Service: blocked(reason=permission or trust)
Service -> Repository: write audit_event(result=blocked, code=3)
Service -> CLI: security blocked (exit 3)
```

## 执行时序（运行故障路径，Draft）
```text
User -> CLI: plugin run --id X --action A
CLI -> Service: parsed request
Service -> PluginRuntime: execute(A)
PluginRuntime -> Service: runtime error
Service -> Repository: write plugin_execution_history(status=failed, code=4)
Service -> CLI: integration failure (exit 4)
```

## 生命周期操作契约（Draft）
1. `install`
- 前置：manifest 通过 schema 校验
- 后置：状态为 `installed`

2. `enable`
- 前置：`trust_status=trusted`
- 后置：状态为 `enabled`

3. `run`
- 前置：`enabled` + 权限校验通过
- 后置：`running -> success/failed`

4. `disable`
- 前置：插件存在
- 后置：状态为 `disabled`

5. `block`
- 触发：签名异常、兼容性不匹配、连续失败策略触发
- 后置：状态为 `blocked`

## 错误码映射（Draft）
- `2`：manifest/action 参数无效
- `3`：信任或权限策略阻断
- `4`：插件运行时/集成失败
- `10`：批量动作部分成功

## 生命周期决策（Phase 1 Freeze）
1. 自动 `blocked` 阈值：5 次连续失败 / 10 分钟窗口触发。
2. `blocked -> enabled`：必须二次审批后才允许恢复。
3. 批量执行重试：采用“逐项重试 + 指数退避”，最大 3 次后进入部分失败汇总。

## 更新规则
- 生命周期语义改动必须同步：
  - `docs/PLUGIN_SYSTEM.md`
  - `docs/API_SPEC.md`
  - `docs/DATA_MODEL.md`
  - `SECURITY.md`
