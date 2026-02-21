from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from synora.paths import ensure_synora_home


class ConfigRepository:
    def __init__(self, config_path: Path | None = None) -> None:
        base = ensure_synora_home()
        self._config_path = config_path or (base / "config.json")

    @property
    def config_path(self) -> Path:
        return self._config_path

    def init_default(self) -> Path:
        self._config_path.parent.mkdir(parents=True, exist_ok=True)
        if self._config_path.exists():
            return self._config_path

        payload = {
            "log_level": "INFO",
            "quarantine_dir": str(ensure_synora_home() / "quarantine"),
            "allow_apply_updates": False,
        }
        self._config_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
        return self._config_path

    def load(self) -> dict[str, Any]:
        if not self._config_path.exists():
            return {}
        return json.loads(self._config_path.read_text(encoding="utf-8"))
