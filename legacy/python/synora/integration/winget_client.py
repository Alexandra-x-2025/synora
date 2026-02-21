from __future__ import annotations

import json
import platform
from dataclasses import dataclass

from synora.domain.models import SoftwareItem, UpdateItem
from synora.security.guard import SecurityGuard


class WingetIntegrationError(OSError):
    pass


@dataclass(slots=True)
class WingetClient:
    guard: SecurityGuard

    def _supported(self) -> bool:
        return platform.system().lower() == "windows"

    def list_installed(self) -> list[SoftwareItem]:
        if not self._supported():
            return []

        cmd = [
            "winget",
            "list",
            "--source",
            "winget",
            "--accept-source-agreements",
            "--output",
            "json",
        ]
        result = self.guard.run(cmd)
        if result.code != 0:
            raise WingetIntegrationError(
                f"winget list failed with code {result.code}: {result.stderr.strip() or 'no stderr'}"
            )

        rows = _parse_json_items(result.stdout, operation="list")
        return [
            SoftwareItem(
                name=row.get("Name", ""),
                package_id=row.get("Id", ""),
                version=row.get("Version", ""),
                source=row.get("Source", ""),
            )
            for row in rows
        ]

    def list_upgrades(self) -> list[UpdateItem]:
        if not self._supported():
            return []

        cmd = [
            "winget",
            "upgrade",
            "--source",
            "winget",
            "--accept-source-agreements",
            "--output",
            "json",
        ]
        result = self.guard.run(cmd)
        if result.code != 0:
            raise WingetIntegrationError(
                f"winget upgrade failed with code {result.code}: {result.stderr.strip() or 'no stderr'}"
            )

        rows = _parse_json_items(result.stdout, operation="upgrade")
        updates: list[UpdateItem] = []
        for row in rows:
            updates.append(
                UpdateItem(
                    name=row.get("Name", ""),
                    package_id=row.get("Id", ""),
                    installed_version=row.get("Version", ""),
                    available_version=row.get("AvailableVersion", ""),
                    source=row.get("Source", ""),
                )
            )
        return updates


def _parse_json_items(raw: str, operation: str) -> list[dict[str, str]]:
    raw = raw.strip()
    if not raw:
        return []

    try:
        payload = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise WingetIntegrationError(f"winget {operation} returned malformed JSON") from exc

    if isinstance(payload, dict):
        if isinstance(payload.get("Sources"), list):
            items: list[dict[str, str]] = []
            for source in payload["Sources"]:
                packages = source.get("Packages", [])
                if isinstance(packages, list):
                    for pkg in packages:
                        if isinstance(pkg, dict):
                            items.append(pkg)
            return items
        if isinstance(payload.get("Data"), list):
            return [row for row in payload["Data"] if isinstance(row, dict)]

    if isinstance(payload, list):
        return [row for row in payload if isinstance(row, dict)]

    raise WingetIntegrationError(f"winget {operation} returned unsupported JSON structure")
