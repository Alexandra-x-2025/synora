from __future__ import annotations

from synora.domain.models import UpdateItem
from synora.domain.risk import RiskLevel, classify_update_risk
from synora.integration.winget_client import WingetClient


class UpdateService:
    def __init__(self, winget: WingetClient) -> None:
        self._winget = winget

    def check_updates(self) -> list[UpdateItem]:
        return self._winget.list_upgrades()

    def plan_apply(self, package_id: str, confirmed: bool) -> RiskLevel:
        if not package_id.strip():
            raise ValueError("package_id is required")
        return classify_update_risk(explicit_confirmed=confirmed)
