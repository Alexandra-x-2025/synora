from __future__ import annotations

from enum import Enum


class RiskLevel(str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"


def classify_update_risk(explicit_confirmed: bool) -> RiskLevel:
    return RiskLevel.LOW if explicit_confirmed else RiskLevel.MEDIUM
