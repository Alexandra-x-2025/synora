from __future__ import annotations

from collections.abc import Callable
from time import sleep
from typing import TypeVar

T = TypeVar("T")


class TaskEngine:
    def run_with_retry(self, fn: Callable[[], T], retries: int = 1, delay_seconds: float = 0.0) -> T:
        attempts = retries + 1
        last_error: Exception | None = None
        for _ in range(attempts):
            try:
                return fn()
            except Exception as exc:  # pragma: no cover - fallback path
                last_error = exc
                if delay_seconds > 0:
                    sleep(delay_seconds)
        assert last_error is not None
        raise last_error
