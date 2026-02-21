from __future__ import annotations

from synora.domain.models import SoftwareItem
from synora.integration.winget_client import WingetClient


class SoftwareService:
    def __init__(self, winget: WingetClient) -> None:
        self._winget = winget

    def list_software(self) -> list[SoftwareItem]:
        return self._winget.list_installed()
