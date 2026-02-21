from __future__ import annotations

import os
from pathlib import Path


def get_synora_home() -> Path:
    env_home = os.getenv("SYNORA_HOME")
    if env_home:
        return Path(env_home).expanduser()
    return Path.home() / ".synora"


def ensure_synora_home() -> Path:
    preferred = get_synora_home()
    try:
        preferred.mkdir(parents=True, exist_ok=True)
        return preferred
    except PermissionError:
        fallback = Path.cwd() / ".synora"
        fallback.mkdir(parents=True, exist_ok=True)
        return fallback
