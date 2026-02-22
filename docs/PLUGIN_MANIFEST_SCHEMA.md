# PLUGIN_MANIFEST_SCHEMA

## 文档目的
定义插件 `manifest.json` 的结构约束，作为插件加载与校验的统一标准。

## 当前状态
- 状态：v0.1 Draft（未冻结）
- 作用范围：Synora 插件系统

## Schema（Draft）
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://synora.local/schema/plugin-manifest-1.0.json",
  "title": "Synora Plugin Manifest",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schema_version",
    "plugin_id",
    "name",
    "version",
    "kind",
    "entry",
    "runtime",
    "api_compat",
    "permissions",
    "actions"
  ],
  "properties": {
    "schema_version": {
      "type": "string",
      "const": "1.0"
    },
    "plugin_id": {
      "type": "string",
      "pattern": "^[a-z0-9]+(\\.[a-z0-9_-]+)+$",
      "minLength": 3,
      "maxLength": 128
    },
    "name": {
      "type": "string",
      "minLength": 1,
      "maxLength": 128
    },
    "version": {
      "type": "string",
      "pattern": "^[0-9]+\\.[0-9]+\\.[0-9]+(-[a-zA-Z0-9.-]+)?$"
    },
    "kind": {
      "type": "string",
      "enum": ["source_provider", "update_policy", "ai_tool", "system_tool"]
    },
    "entry": {
      "type": "string",
      "minLength": 1,
      "maxLength": 256
    },
    "runtime": {
      "type": "string",
      "enum": ["native", "wasm"]
    },
    "api_compat": {
      "type": "string",
      "minLength": 1,
      "maxLength": 64
    },
    "permissions": {
      "type": "array",
      "minItems": 1,
      "uniqueItems": true,
      "items": {
        "type": "string",
        "pattern": "^[a-z][a-z0-9_]*\\.[a-z][a-z0-9_]*\\.[a-z][a-z0-9_]*$"
      }
    },
    "actions": {
      "type": "array",
      "minItems": 1,
      "uniqueItems": true,
      "items": {
        "type": "string",
        "pattern": "^[a-z][a-z0-9_]*$",
        "minLength": 3,
        "maxLength": 64
      }
    },
    "signature": {
      "type": "string",
      "minLength": 16,
      "maxLength": 8192
    },
    "publisher": {
      "type": "string",
      "minLength": 1,
      "maxLength": 128
    },
    "homepage": {
      "type": "string",
      "format": "uri"
    },
    "description": {
      "type": "string",
      "maxLength": 2048
    }
  }
}
```

## 校验规则（Draft）
1. `plugin_id` 必须全局唯一，建议 `<vendor>.<name>`。
2. `runtime=wasm` 时，`entry` 建议以 `.wasm` 结尾。
3. `signature` 在开发态可选；生产态建议必填。
4. `actions` 与 `permissions` 采用最小声明，不可空。

## 示例（Draft）
```json
{
  "schema_version": "1.0",
  "plugin_id": "github.source",
  "name": "GitHub Source Provider",
  "version": "0.1.0",
  "kind": "source_provider",
  "entry": "plugins/github.wasm",
  "runtime": "wasm",
  "api_compat": ">=0.1,<0.2",
  "permissions": ["net.http.read", "inventory.read", "candidate.write"],
  "actions": ["discover_sources", "enrich_candidate"],
  "publisher": "synora-official"
}
```

## 更新规则
- Schema 字段改动必须同步：
  - `docs/PLUGIN_SYSTEM.md`
  - `docs/API_SPEC.md`
  - `docs/DATA_MODEL.md`
