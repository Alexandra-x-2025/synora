from __future__ import annotations

import logging
from pathlib import Path

from synora.paths import ensure_synora_home


def setup_logging() -> Path:
    log_dir = ensure_synora_home() / "logs"
    log_dir.mkdir(parents=True, exist_ok=True)
    log_path = log_dir / "synora.log"

    logger = logging.getLogger("synora")
    logger.setLevel(logging.INFO)

    if not logger.handlers:
        handler = logging.FileHandler(log_path, encoding="utf-8")
        handler.setFormatter(logging.Formatter("%(asctime)s %(levelname)s %(message)s"))
        logger.addHandler(handler)

    return log_path
